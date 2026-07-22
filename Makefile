# Builds the program "combination-lock" from source
VERSION=1.0
NAME=rust-makefile
EXEC=rust-exec
PREFIX=$(HOME)/.local

default: build

build:
	@cargo build --release
clean:
	@rm -rf target/*
	@cargo clean

.PHONY: test
test:
	@target/release/combination-lock
