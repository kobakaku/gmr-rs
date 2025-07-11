use anyhow::Result;
use std::env;

use gmw_rs::{Circuit, GmwProtocol, LocalEvaluator};

/// Run a circuit with unified interface
fn run_circuit(circuit_file: &str, inputs: Vec<bool>, party_count: usize) -> Result<()> {
    let circuit = Circuit::from_file(circuit_file)?;

    // Create GMW protocol instance and run circuit
    let protocol = GmwProtocol::new(party_count)?;
    let outputs = protocol.run_circuit(&circuit, &inputs)?;

    println!("Inputs: {inputs:?}");
    println!("Outputs:");

    for (name, result) in outputs {
        print!("  {name} = {result}");

        // Always verify using local circuit evaluation
        let output_info = circuit
            .metadata
            .outputs
            .iter()
            .find(|info| info.name == name)
            .ok_or_else(|| anyhow::anyhow!("Output {} not found", name))?;

        let expected = LocalEvaluator::get_output(&circuit, &inputs, output_info.id)?;
        if result == expected {
            println!(" ✓");
        } else {
            println!(" ✗ (expected {expected})");
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Usage: cargo run -- [--parties N] <circuit.json> <input1> [input2] [input3] ...");
    println!();
    println!("Options:");
    println!("  --parties N    Use N-party computation (default: 2)");
    println!();
    println!("Examples:");
    println!("  cargo run -- circuits/not.json 1");
    println!("  cargo run -- circuits/and.json 1 0");
    println!("  cargo run -- circuits/half_adder.json 1 1");
    println!("  cargo run -- --parties 3 circuits/and.json 1 0");
    println!("  cargo run -- --parties 4 circuits/xor.json 1 0");
    println!("  cargo run -- --parties 5 circuits/and.json 1 1");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    // Parse command line arguments
    let mut party_count = 2; // Default to 2-party
    let mut arg_idx = 1;

    // Check for --parties flag
    if args.len() > 1 && args[1] == "--parties" && args.len() > 2 {
        party_count = args[2]
            .parse::<usize>()
            .map_err(|_| anyhow::anyhow!("Invalid party count: {}", args[2]))?;
        arg_idx = 3;
    }

    let remaining_args = &args[arg_idx..];

    if remaining_args.is_empty() {
        print_usage();
        return Ok(());
    }

    let circuit_file = &remaining_args[0];

    // Parse all remaining arguments as boolean inputs
    let inputs: Result<Vec<bool>, _> = remaining_args[1..]
        .iter()
        .map(|s| s.parse::<u8>().map(|v| v != 0))
        .collect();

    let inputs = inputs?;

    if inputs.is_empty() && !circuit_file.contains("help") {
        println!("Warning: No inputs provided");
    }

    run_circuit(circuit_file, inputs, party_count)
}
