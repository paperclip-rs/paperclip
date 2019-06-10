
all: fmt build test

clean:
	rm -rf Cargo.lock
	rm -rf openapi/tests/test_k8s/Cargo.lock
	if [ -d "target" ]; then \
		rm -rf target ; \
	fi
	if [ -d "openapi/tests/test_k8s/target" ]; then \
		rm -rf openapi/tests/test_k8s/target ; \
	fi

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

test:
	cargo clippy --all -- -D clippy::all
	cargo test --all --all-features
	# Compile the code generated through tests.
	cd openapi/tests/test_k8s && cargo build
