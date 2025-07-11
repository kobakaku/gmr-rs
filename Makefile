# GMW Protocol Implementation Makefile
# Usage: make <target> [PARTIES=N]
# Default: PARTIES=2
PARTIES ?= 2

.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make xor        - Run XOR circuit test"
	@echo "  make not        - Run NOT circuit test"
	@echo "  make and        - Run AND circuit test with OT"
	@echo "  make or         - Run OR circuit test"
	@echo "  make half-adder - Run half adder circuit test"
	@echo "  make full-adder - Run full adder circuit test"
	@echo "  make equality   - Run 2-bit equality circuit test"
	@echo "  make mux        - Run 2-to-1 multiplexer circuit test"
	@echo "  make test       - Run all unit tests"
	@echo "  make build      - Build the project"
	@echo "  make clean      - Clean build artifacts"
	@echo ""
	@echo "Usage: make <target> [PARTIES=N]"
	@echo "  Default: PARTIES=2"
	@echo "  Example: make and PARTIES=3"
	@echo "  Example: make equality PARTIES=4"

# Build the project
.PHONY: build
build:
	cargo build

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Run all unit tests
.PHONY: test
test:
	cargo test

# Run XOR circuit with all combinations
.PHONY: xor
xor: build
	@echo "======== XOR Circuit ($(PARTIES) parties) ========"
	@cargo run --quiet -- --parties $(PARTIES) circuits/xor.json 0 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/xor.json 0 1
	@cargo run --quiet -- --parties $(PARTIES) circuits/xor.json 1 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/xor.json 1 1

# Run NOT circuit
.PHONY: not
not: build
	@echo "======== NOT Circuit ($(PARTIES) parties) ========"
	@cargo run --quiet -- --parties $(PARTIES) circuits/not.json 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/not.json 1

# Run AND circuit
.PHONY: and
and: build
	@echo "======== AND Circuit ($(PARTIES) parties) ========"
	@cargo run --quiet -- --parties $(PARTIES) circuits/and.json 0 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/and.json 0 1
	@cargo run --quiet -- --parties $(PARTIES) circuits/and.json 1 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/and.json 1 1

# Run OR circuit
.PHONY: or
or: build
	@echo "======== OR Circuit ($(PARTIES) parties) ========"
	@cargo run --quiet -- --parties $(PARTIES) circuits/or.json 0 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/or.json 0 1
	@cargo run --quiet -- --parties $(PARTIES) circuits/or.json 1 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/or.json 1 1

# Run half adder circuit
.PHONY: half-adder
half-adder: build
	@echo "======== Half Adder Circuit ($(PARTIES) parties) ========"
	@cargo run --quiet -- --parties $(PARTIES) circuits/half_adder.json 0 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/half_adder.json 0 1
	@cargo run --quiet -- --parties $(PARTIES) circuits/half_adder.json 1 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/half_adder.json 1 1

# Run full adder circuit
.PHONY: full-adder
full-adder: build
	@echo "======== Full Adder Circuit ($(PARTIES) parties) ========"
	@cargo run --quiet -- --parties $(PARTIES) circuits/full_adder.json 0 0 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/full_adder.json 0 0 1
	@cargo run --quiet -- --parties $(PARTIES) circuits/full_adder.json 0 1 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/full_adder.json 0 1 1
	@cargo run --quiet -- --parties $(PARTIES) circuits/full_adder.json 1 0 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/full_adder.json 1 0 1
	@cargo run --quiet -- --parties $(PARTIES) circuits/full_adder.json 1 1 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/full_adder.json 1 1 1

# Run 2-bit equality circuit
.PHONY: equality
equality: build
	@echo "======== 2-bit Equality Circuit ($(PARTIES) parties) ========"
	@echo "Testing: 00 == 00"
	@cargo run --quiet -- --parties $(PARTIES) circuits/two_bit_equality.json 0 0 0 0
	@echo "Testing: 01 == 01"
	@cargo run --quiet -- --parties $(PARTIES) circuits/two_bit_equality.json 0 1 0 1
	@echo "Testing: 10 == 10"
	@cargo run --quiet -- --parties $(PARTIES) circuits/two_bit_equality.json 1 0 1 0
	@echo "Testing: 11 == 11"
	@cargo run --quiet -- --parties $(PARTIES) circuits/two_bit_equality.json 1 1 1 1
	@echo "Testing: 00 != 11"
	@cargo run --quiet -- --parties $(PARTIES) circuits/two_bit_equality.json 0 0 1 1
	@echo "Testing: 10 != 01"
	@cargo run --quiet -- --parties $(PARTIES) circuits/two_bit_equality.json 1 0 0 1

# Run 2-to-1 multiplexer circuit
.PHONY: mux
mux: build
	@echo "======== 2-to-1 Multiplexer Circuit ($(PARTIES) parties) ========"
	@echo "sel=0: select a"
	@cargo run --quiet -- --parties $(PARTIES) circuits/mux_2to1.json 0 0 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/mux_2to1.json 1 0 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/mux_2to1.json 0 1 0
	@cargo run --quiet -- --parties $(PARTIES) circuits/mux_2to1.json 1 1 0
	@echo "sel=1: select b"
	@cargo run --quiet -- --parties $(PARTIES) circuits/mux_2to1.json 
	@cargo run --quiet -- --parties $(PARTIES) circuits/mux_2to1.json 1 0 1
	@cargo run --quiet -- --parties $(PARTIES) circuits/mux_2to1.json 0 1 1
	@cargo run --quiet -- --parties $(PARTIES) circuits/mux_2to1.json 1 1 1
