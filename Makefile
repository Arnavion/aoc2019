.PHONY: default run test

default:
	cargo build --release

run:
	cargo run --release

test:
	cargo test --release
	cargo clippy --release
