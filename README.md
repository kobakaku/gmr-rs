# GMW-RS

A simplified implementation of the GMW (Goldreich-Micali-Wigderson) protocol for secure multi-party computation in Rust. This implementation supports boolean circuits with XOR and NOT gates only, using XOR-based secret sharing without requiring Oblivious Transfer.

## Generation and References

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>

## Features

- **Boolean Circuit Support**: Processes circuits defined in JSON format
- **XOR and NOT Gates**: Simplified gate set (no OT required)
- **XOR-based Secret Sharing**: Efficient 2-party secret sharing scheme
- **JSON Circuit Parser**: Compatible with garbled-circuit-rs format
- **Educational Implementation**: Clean, readable code for learning GMW concepts

## Current Limitations

- **Limited Gate Set**: Only XOR and NOT gates are supported (no AND, OR)
- **Two-Party Only**: Currently supports exactly 2 parties (Alice and Bob)
- **No Network Communication**: Parties operate in single process
- **Simplified Security**: Not cryptographically secure for production use
- **Educational Purpose**: This is a learning implementation, not production-ready GMW

## Architecture

```
src/
├── circuit/
│   ├── types.rs      # Circuit and gate type definitions
│   ├── parser.rs     # JSON circuit file parser
│   └── mod.rs        # Module exports
├── gates/
│   ├── xor.rs        # XOR gate implementation
│   ├── not.rs        # NOT gate implementation
│   └── mod.rs        # Gate module exports
├── party.rs          # Party structure and circuit evaluation
├── lib.rs            # Library exports
└── main.rs           # Example execution
```

## Usage

### Basic Example

```bash
# Run the default example circuit
cargo run

# Expected output:
# GMW Protocol Implementation - XOR and NOT gates only
# Loaded circuit file: simple_xor_not
# Running circuit: XOR_NOT_example
# Alice input: true
# Bob input: false
# Final result: false
# Expected result: false (NOT(XOR(true, false)))
# Result matches: true
```

### Circuit Format

Circuits are defined in JSON format in the `circuits/` directory:

```json
{
  "name": "simple_xor_not",
  "circuits": [
    {
      "id": "XOR_NOT_example",
      "alice": [1],
      "bob": [2],
      "out": [4],
      "gates": [
        {"id": 3, "type": "XOR", "in": [1, 2]},
        {"id": 4, "type": "NOT", "in": [3]}
      ]
    }
  ]
}
```

### Protocol Details

The GMW protocol implementation follows these steps:

1. **Input Secret Sharing**: Each party's input is split into random shares using XOR
2. **Gate Evaluation**: 
   - **XOR Gate**: Standard GMW - Both parties compute `share_a XOR share_b` locally (no communication needed)
   - **NOT Gate**: Standard GMW - Party 0 computes `NOT share`, Party 1 outputs `share`
3. **Circuit Evaluation**: Gates are processed in topological order
4. **Output Reconstruction**: Final result is reconstructed by XORing both parties' output shares

## Examples

### Example 1: XOR Gate Only
```json
{
  "id": "XOR_only",
  "alice": [1],
  "bob": [2], 
  "out": [3],
  "gates": [
    {"id": 3, "type": "OR", "in": [1, 2]}
  ]
}
```

### Example 2: NOT Gate Only
```json
{
  "id": "NOT_only",
  "alice": [1],
  "out": [2],
  "gates": [
    {"id": 2, "type": "NOT", "in": [1]}
  ]
}
```

## Testing

Run the implementation with:

```bash
# Build and run
cargo run

# Run tests (if available)
cargo test

# Check for compilation errors
cargo check
```

## Implementation Notes

- **Secret Sharing**: Uses `(share1, share2)` where `original = share1 XOR share2`
- **XOR Gate Security**: XOR gates are naturally secure in GMW protocol as they only require local XOR operations on shares
- **True GMW Implementation**: This follows standard GMW protocol for XOR and NOT gates
- **No Communication**: All computation happens locally for demonstration
- **Random Shares**: Uses `rand` crate for generating random shares

## License

This project is for educational and research purposes only. Not intended for production use.
