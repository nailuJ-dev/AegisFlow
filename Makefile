.PHONY: fmt lint test doc audit deny verify run docker

fmt:
<<<<<<< HEAD
<<<<<<< HEAD
	cargo fmt --all
=======
	cargo fmt --all -- --check
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)
=======
	cargo fmt --all
>>>>>>> e1d31e7 (chore: harden runtime, deployment, and release pipeline)

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

doc:
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

audit:
	cargo audit

deny:
	cargo deny check

verify: fmt lint test doc

run:
	cargo run -p aegisflow-service

docker:
	docker build -t aegisflow:dev .
