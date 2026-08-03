#!/usr/bin/env bash
set -euo pipefail
cargo run -p aegisflow-service &
pid=$!
trap 'kill "$pid" 2>/dev/null || true' EXIT
sleep 2
curl --fail --silent http://127.0.0.1:8080/healthz
