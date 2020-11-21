
all: fmt build test

clean:
	rm -rf Cargo.lock
	rm -rf target
	rm -rf tests/test_k8s
	git checkout tests/test_k8s
	rm -rf tests/test_pet

prepare:
	rustup override set stable
	rustup component add rustfmt
	rustup component add clippy

fmt:
	cargo fmt --all

doc:
	cargo doc --all --all-features --no-deps

build:
	cargo build
	cargo build --features actix
	cargo build --features cli

test:
	cargo clippy --all --features "actix" -- -D clippy::all
	cargo test --all --features "actix cli chrono uuid"
	# Compile the code generated through tests.
	cd tests/test_pet && cargo check
	cd tests/test_pet/cli && CARGO_TARGET_DIR=../target cargo check
	cd tests/test_k8s && cargo check
	cd tests/test_k8s/cli && CARGO_TARGET_DIR=../target cargo check
	# Test that the CLI runs successfully.
	# ./tests/test_k8s/target/debug/test-k8s-cli --help > /dev/null
