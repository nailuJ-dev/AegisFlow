.PHONY: fmt lint test doc audit deny verify run docker

fmt:
<<<<<<< HEAD
	cargo fmt --all
=======
	cargo fmt --all -- --check
>>>>>>> 2081585 (feat: initialize production-ready Rust scaffold)

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
