# GMW Protocol Implementation Makefile
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
	@echo "  make all        - Run all basic gate tests"
	@echo "  make multi      - Run all multi-gate circuit tests"
	@echo "  make build      - Build the project"
	@echo "  make clean      - Clean build artifacts"

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

# Run half adder circuit
.PHONY: half-adder
half-adder: build
	@echo "======== Half Adder Circuit ========"
	@cargo run --quiet -- circuits/half_adder.json 0 0
	@cargo run --quiet -- circuits/half_adder.json 0 1
	@cargo run --quiet -- circuits/half_adder.json 1 0
	@cargo run --quiet -- circuits/half_adder.json 1 1

# Run full adder circuit
.PHONY: full-adder
full-adder: build
	@echo "======== Full Adder Circuit ========"
	@cargo run --quiet -- circuits/full_adder.json 0 0 0
	@cargo run --quiet -- circuits/full_adder.json 0 0 1
	@cargo run --quiet -- circuits/full_adder.json 0 1 0
	@cargo run --quiet -- circuits/full_adder.json 0 1 1
	@cargo run --quiet -- circuits/full_adder.json 1 0 0
	@cargo run --quiet -- circuits/full_adder.json 1 0 1
	@cargo run --quiet -- circuits/full_adder.json 1 1 0
	@cargo run --quiet -- circuits/full_adder.json 1 1 1

# Run 2-bit equality circuit
.PHONY: equality
equality: build
	@echo "======== 2-bit Equality Circuit ========"
	@echo "Testing: 00 == 00"
	@cargo run --quiet -- circuits/two_bit_equality.json 0 0 0 0
	@echo "Testing: 01 == 01"
	@cargo run --quiet -- circuits/two_bit_equality.json 0 1 0 1
	@echo "Testing: 10 == 10"
	@cargo run --quiet -- circuits/two_bit_equality.json 1 0 1 0
	@echo "Testing: 11 == 11"
	@cargo run --quiet -- circuits/two_bit_equality.json 1 1 1 1
	@echo "Testing: 00 != 11"
	@cargo run --quiet -- circuits/two_bit_equality.json 0 0 1 1
	@echo "Testing: 10 != 01"
	@cargo run --quiet -- circuits/two_bit_equality.json 1 0 0 1

# Run 2-to-1 multiplexer circuit
.PHONY: mux
mux: build
	@echo "======== 2-to-1 Multiplexer Circuit ========"
	@echo "sel=0: select a"
	@cargo run --quiet -- circuits/mux_2to1.json 0 0 0
	@cargo run --quiet -- circuits/mux_2to1.json 1 0 0
	@cargo run --quiet -- circuits/mux_2to1.json 0 1 0
	@cargo run --quiet -- circuits/mux_2to1.json 1 1 0
	@echo "sel=1: select b"
	@cargo run --quiet -- circuits/mux_2to1.json 0 0 1
	@cargo run --quiet -- circuits/mux_2to1.json 1 0 1
	@cargo run --quiet -- circuits/mux_2to1.json 0 1 1
	@cargo run --quiet -- circuits/mux_2to1.json 1 1 1

# Run all basic gate tests
.PHONY: all
all: xor not and or

# Run all multi-gate circuit tests
.PHONY: multi
multi: half-adder full-adder equality mux