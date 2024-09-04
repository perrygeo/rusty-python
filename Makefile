default:
	@echo "Usage: make build|test"

build:
	maturin build --release

dev:
	maturin develop

test-rust:
	cd rust && cargo test

test-python:
	python -m pytest --verbose

test: dev test-rust test-python

