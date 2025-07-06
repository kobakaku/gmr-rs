# GMW Protocol Implementation Makefile
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make xor      - Run XOR circuit test"
	@echo "  make not      - Run NOT circuit test"
	@echo "  make and      - Run AND circuit test with OT"
	@echo "  make or       - Run OR circuit test"
	@echo "  make all      - Run all tests"
	@echo "  make build    - Build the project"
	@echo "  make clean    - Clean build artifacts"

# Build the project
.PHONY: build
build:
	cargo build

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Run XOR circuit with all combinations
.PHONY: xor
xor: build
	@echo "======== XOR Circuit ========"
	@cargo run --quiet -- circuits/xor.json 0 0
	@cargo run --quiet -- circuits/xor.json 0 1
	@cargo run --quiet -- circuits/xor.json 1 0
	@cargo run --quiet -- circuits/xor.json 1 1

# Run NOT circuit
.PHONY: not
not: build
	@echo "======== NOT Circuit ========"
	@cargo run --quiet -- circuits/not.json 0
	@cargo run --quiet -- circuits/not.json 1

# Run AND circuit
.PHONY: and
and: build
	@echo "======== AND Circuit ========"
	@cargo run --quiet -- circuits/and.json 0 0
	@cargo run --quiet -- circuits/and.json 0 1
	@cargo run --quiet -- circuits/and.json 1 0
	@cargo run --quiet -- circuits/and.json 1 1

# Run OR circuit
.PHONY: or
or: build
	@echo "======== OR Circuit ========"
	@cargo run --quiet -- circuits/or.json 0 0
	@cargo run --quiet -- circuits/or.json 0 1
	@cargo run --quiet -- circuits/or.json 1 0
	@cargo run --quiet -- circuits/or.json 1 1

# Run all tests
.PHONY: all
all: xor not and or