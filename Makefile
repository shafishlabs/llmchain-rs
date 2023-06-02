.PHONY: check build test integration

default: build

lint:
	cargo fmt --all
	cargo clippy --all-targets --all-features -- -D warnings
	# Unused deps
	cargo machete

build:
	cargo build

test:
	cargo test
