
all: fmt build test

clean:
	if [ -d "target" ]; then \
		rm -rf target ; \
	fi

prepare:
	rustup component add rustfmt
	rustup component add clippy

fmt:
	cargo fmt --all

build:
	cargo build

test:
	cargo clippy --all -- -D clippy::all
	cargo test --all
