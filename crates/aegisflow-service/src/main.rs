use std::{env, net::SocketAddr, sync::Arc};

    use anyhow::Context;
    use axum::{
        extract::{DefaultBodyLimit, State},
        http::StatusCode,
        response::IntoResponse,
        routing::{get, post},
        Json, Router,
    };
    use serde::Serialize;
    use tower_http::{
        catch_panic::CatchPanicLayer,
        request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
        trace::TraceLayer,
    };
    use tracing::info;
    use aegisflow_core::{Capability, DataLabel, Operation, PolicyEngine, ToolRequest};
use serde::Deserialize;

    #[derive(Clone)]
    struct AppState {

        started: std::time::Instant,
    }

    #[derive(Debug, Serialize)]
    struct Health<'a> {
        status: &'a str,
        version: &'a str,
        uptime_seconds: u64,
    }

    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
        init_tracing();
        let bind: SocketAddr = env::var("APP_BIND")
            .unwrap_or_else(|_| "0.0.0.0:8080".to_owned())
            .parse()
            .context("APP_BIND must be a socket address")?;
        let max_body_bytes: usize = env::var("APP_MAX_BODY_BYTES")
            .unwrap_or_else(|_| "1048576".to_owned())
            .parse()
            .context("APP_MAX_BODY_BYTES must be a positive integer")?;
        anyhow::ensure!(max_body_bytes > 0, "APP_MAX_BODY_BYTES must be greater than zero");

        let state = Arc::new(AppState {

            started: std::time::Instant::now(),
        });

        let app = Router::new()
            .route("/healthz", get(health))
            .route("/readyz", get(health))
            .route("/v1/evaluate", post(handle))
            .with_state(state)
            .layer(DefaultBodyLimit::max(max_body_bytes))
            .layer(PropagateRequestIdLayer::x_request_id())
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            .layer(CatchPanicLayer::new())
            .layer(TraceLayer::new_for_http());

        let listener = tokio::net::TcpListener::bind(bind)
            .await
            .with_context(|| format!("failed to bind {bind}"))?;
        info!(%bind, "service started");
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .context("HTTP server failed")
    }

    async fn health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        Json(Health {
            status: "ok",
            version: env!("CARGO_PKG_VERSION"),
            uptime_seconds: state.started.elapsed().as_secs(),
        })
    }

    async fn handle(
        State(state): State<Arc<AppState>>,
        Json(request): Json<Request>,
    ) -> Result<impl IntoResponse, ApiError> {
        let _ = state;
        let response = evaluate(request)?;
        Ok((StatusCode::OK, Json(response)))
    }


    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Request {
        operation: Operation,
        label: DataLabel,
        argument: String,
        subject: Option<String>,
        capability_ttl_seconds: Option<u64>,
    }

    fn evaluate(request: Request) -> anyhow::Result<serde_json::Value> {
        let capabilities = match request.capability_ttl_seconds {
            Some(ttl) => vec![Capability::issue(
                request.operation,
                request.subject.unwrap_or_else(|| "api-workflow".to_owned()),
                ttl,
            )?],
            None => Vec::new(),
        };
        let tool_request = ToolRequest::new(request.operation, request.label, request.argument);
        let decision = PolicyEngine.evaluate(&tool_request, &capabilities);
        Ok(serde_json::to_value(decision)?)
    }

    #[derive(Debug)]
    struct ApiError(anyhow::Error);

    impl<E> From<E> for ApiError
    where
        E: Into<anyhow::Error>,
    {
        fn from(value: E) -> Self {
            Self(value.into())
        }
    }

    impl IntoResponse for ApiError {
        fn into_response(self) -> axum::response::Response {
            tracing::warn!(error = %self.0, "request rejected");
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({"error": "request could not be processed"})),
            )
                .into_response()
        }
    }

    fn init_tracing() {
        let filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .init();
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            if let Err(error) = tokio::signal::ctrl_c().await {
                tracing::error!(%error, "failed to install Ctrl+C handler");
            }
        };
        #[cfg(unix)]
        let terminate = async {
            match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
                Ok(mut signal) => {
                    signal.recv().await;
                }
                Err(error) => tracing::error!(%error, "failed to install SIGTERM handler"),
            }
        };
        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            () = ctrl_c => {},
            () = terminate => {},
        }
        info!("shutdown signal received");
    }
