# GMW-RS

A complete implementation of the GMW (Goldreich-Micali-Wigderson) protocol for secure multi-party computation in Rust. This implementation supports boolean circuits with XOR, NOT, AND, and OR gates for arbitrary number of parties (n ≥ 2), using XOR-based secret sharing with Oblivious Transfer for AND/OR gates.

## Generation and References

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>

## Features

- **Complete Boolean Circuit Support**: XOR, NOT, AND, and OR gates
- **N-Party Computation**: Supports arbitrary number of parties (n ≥ 2)
- **Oblivious Transfer Integration**: Uses [oblivious-transfer-rs](https://github.com/kobakaku/oblivious-transfer-rs) for AND/OR gates
- **XOR-based Secret Sharing**: Efficient n-party secret sharing scheme
- **JSON Circuit Parser**: Compatible with garbled-circuit-rs format
- **Unified Protocol Implementation**: Clean, structured design using `GmwProtocol` struct
- **Simplified Implementation**: Dependency-free design for learning GMW concepts

## Limitations

- **Single Process**: Parties operate within the same process (no network communication)
- **Educational Purpose**: Designed for learning, not production use
- **Semi-Honest Security**: Assumes honest-but-curious adversaries only

## Architecture

```
src/
├── circuit/
│   ├── types.rs      # Circuit and gate type definitions
│   └── mod.rs        # Module exports
├── gates/
│   ├── xor.rs        # XOR gate implementation (local)
│   ├── not.rs        # NOT gate implementation (local)
│   ├── and.rs        # AND gate with Oblivious Transfer
│   ├── or.rs         # OR gate using De Morgan's law
│   └── mod.rs        # Gate module exports
├── ot/
│   └── mod.rs        # OT wrapper for GMW protocol
├── protocol.rs       # GmwProtocol struct with unified implementation
├── lib.rs            # Library exports
└── main.rs           # CLI interface
```

## Usage

### Command Line Interface

```bash
# Single input circuits (NOT gate)
cargo run -- circuits/not.json 1

# Two input circuits (AND, OR, XOR)
cargo run -- circuits/and.json 1 1
cargo run -- circuits/or.json 0 1
cargo run -- circuits/xor.json 1 0

# Specify number of parties (default: 2)
cargo run -- --parties 3 circuits/and.json 1 1
cargo run -- --parties 4 circuits/xor.json 1 0
cargo run -- --parties 5 circuits/or.json 0 1
```

### Makefile Commands

```bash
# Test individual gate types (default: 2 parties)
make not      # Test NOT gate with all inputs
make xor      # Test XOR gate with all combinations
make and      # Test AND gate with OT protocol
make or       # Test OR gate with OT protocol

# Test with specific number of parties
make and PARTIES=3         # Test AND gate with 3 parties
make xor PARTIES=4         # Test XOR gate with 4 parties
make equality PARTIES=5    # Test equality circuit with 5 parties

# Test complex circuits
make half-adder            # Test half adder
make full-adder            # Test full adder
make equality              # Test 2-bit equality
make mux                   # Test multiplexer

# Run all tests
make test                  # Cargo unit tests

# Build project
make build

# Clean artifacts
make clean
```

### Circuit Format

Circuits are defined in JSON format in the `circuits/` directory:

```json
{
  "name": "AND_gate",
  "description": "Simple AND gate with OT",
  "metadata": {
    "inputs": [
      {
        "name": "a",
        "wire_id": 1
      },
      {
        "name": "b", 
        "wire_id": 2
      }
    ],
    "outputs": [
      {
        "name": "result",
        "gate_id": 3
      }
    ]
  },
  "gates": [
    {
      "id": 3,
      "type": "AND",
      "in": [1, 2]
    }
  ]
}
```

### Protocol Details

The GMW protocol implementation follows these steps:

1. **Input Secret Sharing**: Each party's input is split into random shares using XOR: `value = share₀ ⊕ share₁ ⊕ ... ⊕ shareₙ₋₁`
2. **Gate Evaluation**: 
   - **XOR Gate**: Local computation - Each party computes `share_a XOR share_b`
   - **NOT Gate**: Local computation - Party 0 flips bit, other parties keep unchanged
   - **AND Gate**: Requires cross-term computation using Oblivious Transfer between all party pairs
   - **OR Gate**: Uses De Morgan's law: `x | y = ~(~x & ~y)` with OT-based AND
3. **Circuit Evaluation**: Gates are processed in topological order
4. **Output Reconstruction**: Final result is reconstructed by XORing all parties' output shares

## Examples

### Example 1: AND Gate with OT
```json
{
  "name": "AND_gate",
  "description": "AND gate requiring Oblivious Transfer",
  "metadata": {
    "inputs": [
      {"name": "a", "wire_id": 1},
      {"name": "b", "wire_id": 2}
    ],
    "outputs": [
      {"name": "result", "gate_id": 3}
    ]
  },
  "gates": [
    {"id": 3, "type": "AND", "in": [1, 2]}
  ]
}
```

### Example 2: OR Gate with De Morgan's Law
```json
{
  "name": "OR_gate", 
  "description": "OR gate using De Morgan's law",
  "metadata": {
    "inputs": [
      {"name": "a", "wire_id": 1},
      {"name": "b", "wire_id": 2}
    ],
    "outputs": [
      {"name": "result", "gate_id": 3}
    ]
  },
  "gates": [
    {"id": 3, "type": "OR", "in": [1, 2]}
  ]
}
```

## GMW Protocol Implementation

### Secret Sharing
- Uses XOR-based n-party secret sharing: `value = share₀ ⊕ share₁ ⊕ ... ⊕ shareₙ₋₁`
- Random shares generated using `rand::random::<bool>()`
- Last share computed to ensure XOR equals original value

### Gate Implementations

#### Local Gates (No Communication)
- **XOR**: Each party computes `shareᵢ_x ⊕ shareᵢ_y` locally
- **NOT**: Party 0 flips bit, other parties keep shares unchanged

#### Interactive Gates (Require OT)
- **AND**: Requires cross-term computation between all party pairs using OT
  - Each party computes local term: `shareᵢ_x & shareᵢ_y`
  - Cross terms computed via OT: `shareᵢ_x & shareⱼ_y ⊕ shareⱼ_x & shareᵢ_y` for all i,j pairs
- **OR**: Uses De Morgan's law `x|y = ¬(¬x & ¬y)` with OT-based AND

### Oblivious Transfer
- Uses RSA-based 1-out-of-4 OT from [oblivious-transfer-rs](https://github.com/kobakaku/oblivious-transfer-rs)
- Wrapper `BitOT` converts between `bool` and `Vec<u8>` for compatibility
- Each AND gate requires O(n²) OT executions for n parties

## Dependencies

- `rand = "0.8"` - Random number generation for secret shares
- `serde = "1.0"` - JSON serialization for circuit parsing
- `anyhow = "1.0"` - Error handling
- `oblivious-transfer-rs` - Oblivious Transfer implementation

## Current Limitations

### Network Communication
- **Single Process**: All parties operate within the same process
- **No Network Security**: No secure channels between parties
- **Local Simulation**: Simulates multi-party computation locally

### Security Model
- **Semi-Honest Only**: Assumes honest-but-curious adversaries
- **No Malicious Security**: No protection against actively malicious parties
- **Educational Focus**: Not optimized for performance or production use

### Performance
- **O(n²) Complexity**: AND gates require quadratic number of OT operations
- **Synchronous Execution**: All parties execute simultaneously
- **No Optimizations**: Basic implementation without performance tuning

## Security Note

This is an educational implementation designed for learning the GMW protocol. It includes:
- ✅ Correct GMW protocol logic
- ✅ Proper n-party secret sharing  
- ✅ Authentic Oblivious Transfer
- ✅ Complete n-party computation support (n ≥ 2)
- ❌ No network security
- ❌ No protection against malicious adversaries
- ❌ Not production-ready

## License

This project is for educational and research purposes only. Not intended for production use.