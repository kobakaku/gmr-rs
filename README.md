# GMW-RS

A complete implementation of the GMW (Goldreich-Micali-Wigderson) protocol for secure multi-party computation in Rust. This implementation supports boolean circuits with XOR, NOT, AND, and OR gates, using XOR-based secret sharing with Oblivious Transfer for AND/OR gates.

## Generation and References

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>

## Features

- **Complete Boolean Circuit Support**: XOR, NOT, AND, and OR gates
- **Oblivious Transfer Integration**: Uses [oblivious-transfer-rs](https://github.com/kobakaku/oblivious-transfer-rs) for AND/OR gates
- **XOR-based Secret Sharing**: Efficient 2-party secret sharing scheme
- **JSON Circuit Parser**: Compatible with garbled-circuit-rs format
- **Simplified Implementation**: Clean, dependency-free design for learning GMW concepts

## Limitations

- **Two-Party Only**: Currently supports exactly 2 parties (Alice and Bob) - no multi-party support
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
├── party.rs          # Circuit evaluation logic
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
```

### Makefile Commands

```bash
# Test individual gate types
make not      # Test NOT gate with all inputs
make xor      # Test XOR gate with all combinations
make and      # Test AND gate with OT protocol
make or       # Test OR gate with OT protocol

# Run all tests
make all

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

1. **Input Secret Sharing**: Each party's input is split into random shares using XOR
2. **Gate Evaluation**: 
   - **XOR Gate**: Local computation - Both parties compute `share_a XOR share_b`
   - **NOT Gate**: Local computation - Party 0 flips bit, Party 1 keeps unchanged
   - **AND Gate**: Requires 2 rounds of Oblivious Transfer for cross-product terms
   - **OR Gate**: Uses De Morgan's law: `x | y = ~(~x & ~y)` with OT-based AND
3. **Circuit Evaluation**: Gates are processed in topological order
4. **Output Reconstruction**: Final result is reconstructed by XORing both parties' output shares

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

## Testing

Run tests using the Makefile:

```bash
# Test all gate types
make all

# Test specific gates
make and   # Tests: 0&0=0, 0&1=0, 1&0=0, 1&1=1
make or    # Tests: 0|0=0, 0|1=1, 1|0=1, 1|1=1
make xor   # Tests: 0⊕0=0, 0⊕1=1, 1⊕0=1, 1⊕1=0
make not   # Tests: ¬0=1, ¬1=0

# Run cargo tests
cargo test

# Check compilation
cargo check
```

## GMW Protocol Implementation

### Secret Sharing
- Uses XOR-based 2-party secret sharing: `value = share1 ⊕ share2`
- Random shares generated using `rand::random::<bool>()`

### Gate Implementations

#### Local Gates (No Communication)
- **XOR**: `(x0 ⊕ x1) ⊕ (y0 ⊕ y1) = (x0 ⊕ y0) ⊕ (x1 ⊕ y1)`
- **NOT**: Party 0 flips bit, Party 1 keeps unchanged

#### Interactive Gates (Require OT)
- **AND**: Requires 2 OT rounds for cross-product terms `x0&y1` and `x1&y0`
- **OR**: Uses De Morgan's law `x|y = ¬(¬x & ¬y)` with OT-based AND

### Oblivious Transfer
- Uses RSA-based 1-out-of-2 OT from [oblivious-transfer-rs](https://github.com/kobakaku/oblivious-transfer-rs)
- Wrapper `BitOT` converts between `bool` and `Vec<u8>` for compatibility
- Each AND gate requires 2 OT executions (one per cross-product term)

## Dependencies

- `rand = "0.8"` - Random number generation for secret shares
- `serde = "1.0"` - JSON serialization for circuit parsing
- `anyhow = "1.0"` - Error handling
- `oblivious-transfer-rs` - Oblivious Transfer implementation

## Current Limitations

### Multi-Party Support
- **Only 2-Party**: This implementation is specifically designed for 2-party computation (Alice and Bob)
- **No N-Party Extension**: GMW protocol supports arbitrary number of parties, but this implementation does not
- **Hardcoded Party Logic**: All gate implementations assume exactly 2 parties
- **Future Extension**: Adding multi-party support would require:
  - Generalizing secret sharing to n parties
  - Extending OT to multi-party protocols (like BGW)
  - Modifying all gate implementations

### Other Limitations
- **Single Process**: No network communication between parties
- **Semi-Honest Only**: No protection against malicious adversaries
- **Educational Focus**: Not optimized for performance or production use

## Security Note

This is an educational implementation designed for learning the GMW protocol. It includes:
- ✅ Correct GMW protocol logic
- ✅ Proper secret sharing  
- ✅ Authentic Oblivious Transfer
- ✅ 2-party computation support
- ❌ No multi-party (n > 2) support
- ❌ No network security
- ❌ No protection against malicious adversaries
- ❌ Not production-ready

## License

This project is for educational and research purposes only. Not intended for production use.