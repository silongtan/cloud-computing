rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format-check:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet


build-release:
	cargo build --release
	
run:
	cargo run

build:
	docker build -t car .

run-docker:
	docker run -it --rm -p 8080:8080 car



all: format lint test run