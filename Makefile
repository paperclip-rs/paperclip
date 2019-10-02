
all: fmt build test

clean:
	rm -rf Cargo.lock
	rm -rf target
	rm -rf tests/test_k8s
	git checkout tests/test_k8s
	rm -rf tests/test_pet

prepare:
	rustup override set $$(head  -1 ./rust-toolchain)
	rustup component add rustfmt
	rustup component add clippy

fmt:
	cargo fmt --all

doc:
	cargo doc --all --all-features --no-deps

build:
	cargo build
	cargo build --features v2
	cargo build --features datetime
	cargo build --features default
	cargo build --features actix
	cargo build --features cli
	cargo build --features uid
	cargo build --all --all-features

test:
	cargo clippy --all -- -D clippy::all
	cargo test --all --all-features
	# Compile the code generated through tests.
	cd tests/test_pet && cargo check
	cd tests/test_k8s && cargo check
	cd tests/test_k8s/cli && CARGO_TARGET_DIR=../target cargo build
	# Test that the CLI runs successfully.
	./tests/test_k8s/target/debug/test-k8s-cli --help > /dev/null
