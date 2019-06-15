
all: fmt build test

clean:
	rm -rf Cargo.lock
	rm -rf openapi/tests/test_k8s/Cargo.lock
	rm -rf target
	rm -rf openapi/tests/test_k8s/target

prepare:
	rustup override set nightly-2019-06-09
	rustup component add rustfmt
	rustup component add clippy

fmt:
	cargo fmt --all

doc:
	cargo doc --all --all-features --no-deps

build:
	cargo build
	cargo build --features default
	cargo build --all --all-features

test:
	cargo clippy --all -- -D clippy::all
	cargo test --all --all-features
	# Compile the code generated through tests.
	cd openapi/tests/test_k8s && cargo build
