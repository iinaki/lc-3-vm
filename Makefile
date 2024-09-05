all: test clippy fmt example-2048

build:
	cargo build --release
	cargo install --path .

test:
	cargo test

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt

example-2048:
	cargo run examples/2048.obj

example-rogue:
	cargo run examples/rogue.obj
