all: test clippy fmt run

test:
	cargo test

clippy:
	cargo clippy

fmt:
	cargo fmt

run:
	cargo run

example-2048:
	cargo run examples/2048.obj
