FROM rust:1.85-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release -p aegisflow-service

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /src/target/release/aegisflow-service /usr/local/bin/service
ENV RUST_LOG=info
EXPOSE 8080
USER nonroot:nonroot
ENTRYPOINT ["/usr/local/bin/service"]
