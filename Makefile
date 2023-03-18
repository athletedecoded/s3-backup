rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter
	
install:
	pip3 install cargo-lambda

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet
run:
	cargo run 

release:
	cargo lambda build --release

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

all: format lint test run