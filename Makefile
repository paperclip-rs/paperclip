
all: fmt build test

clean:
	if [ -d "target" ]; then \
		rm -rf target ; \
	fi

prepare:
	rustup override set nightly-2019-05-09
	rustup component add rustfmt
	rustup component add clippy

fmt:
	cargo fmt --all

doc:
	cargo doc --all-features

build:
	cargo build

test:
	cargo clippy --all -- -D clippy::all
	cargo test --all --all-features
	cd openapi/tests/test_k8s && cargo build
