use std::{
    env,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use anyhow::Context;
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
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

struct AppState {
    started: std::time::Instant,
    requests: AtomicU64,
    concurrency: tokio::sync::Semaphore,
    request_timeout: Duration,
}

#[derive(Debug, Serialize)]
struct Health<'a> {
    status: &'a str,
    version: &'a str,
    uptime_seconds: u64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    operation: Operation,
    label: DataLabel,
    argument: String,
    subject: String,
    capability_ttl_seconds: Option<u64>,
}

fn process(request: Request) -> anyhow::Result<serde_json::Value> {
    let capabilities = match request.capability_ttl_seconds {
        Some(ttl) => vec![Capability::issue(
            request.operation,
            request.subject.clone(),
            ttl,
        )?],
        None => Vec::new(),
    };
    let tool_request = ToolRequest::new(
        request.subject,
        request.operation,
        request.label,
        request.argument,
    );
    let decision = PolicyEngine.evaluate(&tool_request, &capabilities);
    Ok(serde_json::to_value(decision)?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let bind: SocketAddr = env::var("APP_BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_owned())
        .parse()
        .context("APP_BIND must be a socket address")?;
    let max_body_bytes = parse_bounded_env("APP_MAX_BODY_BYTES", 1_048_576, 1, 16 * 1024 * 1024)?;
    let max_concurrency = parse_bounded_env("APP_MAX_CONCURRENCY", 32, 1, 1024)?;
    let request_timeout_seconds = parse_bounded_env("APP_REQUEST_TIMEOUT_SECONDS", 30, 1, 300)?;

    let state = Arc::new(AppState {
        started: std::time::Instant::now(),
        requests: AtomicU64::new(0),
        concurrency: tokio::sync::Semaphore::new(max_concurrency),
        request_timeout: Duration::from_secs(u64::try_from(request_timeout_seconds)?),
    });

    let app = Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(health))
        .route("/metrics", get(metrics))
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

async fn metrics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let requests = state.requests.load(Ordering::Relaxed);
    let body = format!(
        "# TYPE aegisflow_requests_total counter\naegisflow_requests_total {requests}\n"
    );
    ([(header::CONTENT_TYPE, "text/plain; version=0.0.4")], body)
}

async fn handle(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Request>,
) -> Result<impl IntoResponse, ApiError> {
    let _permit = state
        .concurrency
        .acquire()
        .await
        .map_err(|_| anyhow::anyhow!("request concurrency gate is closed"))?;
    state.requests.fetch_add(1, Ordering::Relaxed);

    let joined = tokio::time::timeout(
        state.request_timeout,
        tokio::task::spawn_blocking(move || process(request)),
    )
    .await
    .map_err(|_| anyhow::anyhow!("request timed out"))?;
    let response = joined.map_err(|error| anyhow::anyhow!("worker task failed: {error}"))??;
    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug)]
struct ApiError(anyhow::Error);

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
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

fn parse_bounded_env(
    name: &'static str,
    default: usize,
    minimum: usize,
    maximum: usize,
) -> anyhow::Result<usize> {
    let value = env::var(name)
        .unwrap_or_else(|_| default.to_string())
        .parse::<usize>()
        .with_context(|| format!("{name} must be a positive integer"))?;
    anyhow::ensure!(
        (minimum..=maximum).contains(&value),
        "{name} must be in {minimum}..={maximum}"
    );
    Ok(value)
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
