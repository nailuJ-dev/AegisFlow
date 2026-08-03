.PHONY: fmt lint test doc audit deny verify run docker

fmt:
	cargo fmt --all -- --check

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
