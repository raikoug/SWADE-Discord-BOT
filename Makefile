.PHONY: fmt check test clippy run

fmt:
	cargo fmt

check:
	cargo check

test:
	cargo test

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

run:
	cargo run
