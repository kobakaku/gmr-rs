use anyhow::Result;
use std::collections::HashMap;
use std::env;

use gmw_rs::{execute_circuit, reconstruct_shares, secret_share, Circuit, LocalEvaluator};

/// Run a circuit with any number of inputs
fn run_circuit(circuit_file: &str, inputs: Vec<bool>) -> Result<()> {
    let circuit = Circuit::from_file(circuit_file)?;

    // Validate input count
    if circuit.metadata.input_count > 0 && inputs.len() != circuit.metadata.input_count {
        return Err(anyhow::anyhow!(
            "Circuit expects {} inputs but got {}",
            circuit.metadata.input_count,
            inputs.len()
        ));
    }

    // Create secret shares for all inputs
    let mut alice_shares = HashMap::new();
    let mut bob_shares = HashMap::new();

    for (i, &input) in inputs.iter().enumerate() {
        let (alice_share, bob_share) = secret_share(input);
        let wire_id = (i + 1) as u32; // Wire IDs start from 1
        alice_shares.insert(wire_id, alice_share);
        bob_shares.insert(wire_id, bob_share);
    }

    // Evaluate circuit
    let (alice_result_shares, bob_result_shares) =
        execute_circuit(&circuit, &alice_shares, &bob_shares)?;

    println!("Inputs: {inputs:?}");

    // Handle outputs using metadata
    if circuit.metadata.outputs.is_empty() {
        return Err(anyhow::anyhow!(
            "Circuit has no output metadata. Please add metadata to the circuit JSON file."
        ));
    }

    println!("Outputs:");
    for output_info in &circuit.metadata.outputs {
        let alice_output = alice_result_shares
            .get(&output_info.gate_id)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Missing output gate {}", output_info.gate_id))?;
        let bob_output = bob_result_shares
            .get(&output_info.gate_id)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Missing output gate {}", output_info.gate_id))?;

        let result = reconstruct_shares(alice_output, bob_output);
        print!("  {} = {}", output_info.name, result);

        // Always verify using local circuit evaluation
        let expected = LocalEvaluator::get_output(&circuit, &inputs, output_info.gate_id)?;
        assert_eq!(result, expected);
        print!(" ✓");
        println!();
    }

    Ok(())
}

fn print_usage() {
    println!("Usage: cargo run -- <circuit.json> <input1> [input2] [input3] ...");
    println!();
    println!("Examples:");
    println!("  cargo run -- circuits/not.json 1");
    println!("  cargo run -- circuits/and.json 1 0");
    println!("  cargo run -- circuits/half_adder.json 1 1");
    println!("  cargo run -- circuits/full_adder.json 1 1 0");
    println!("  cargo run -- circuits/two_bit_equality.json 1 0 1 0");
    println!("  cargo run -- circuits/mux_2to1.json 1 0 1");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let circuit_file = &args[1];

    // Parse all remaining arguments as boolean inputs
    let inputs: Result<Vec<bool>, _> = args[2..]
        .iter()
        .map(|s| s.parse::<u8>().map(|v| v != 0))
        .collect();

    let inputs = inputs?;

    if inputs.is_empty() && !circuit_file.contains("help") {
        println!("Warning: No inputs provided");
    }

    run_circuit(circuit_file, inputs)
}
