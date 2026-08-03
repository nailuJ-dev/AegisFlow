#!/usr/bin/env bash
set -euo pipefail
<<<<<<< HEAD
<<<<<<< HEAD
cargo fmt --all
=======
cargo fmt --all -- --check
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
cargo fmt --all
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
