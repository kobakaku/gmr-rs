use anyhow::Result;
use gmw_rs::{evaluate_circuit_two_party, reconstruct_shares, secret_share, Circuit};
use std::collections::HashMap;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 4 {
        // Format: cargo run -- <circuit_file> <input1> <input2>
        let circuit_file = &args[1];
        let input1 = args[2].parse::<u8>()? != 0;
        let input2 = args[3].parse::<u8>()? != 0;
        run_two_input_circuit(circuit_file, input1, input2)?;
    } else if args.len() == 3 {
        // Format: cargo run -- <circuit_file> <input>
        let circuit_file = &args[1];
        let input = args[2].parse::<u8>()? != 0;
        run_single_input_circuit(circuit_file, input)?;
    } else {
        println!("Usage:");
        println!("  cargo run -- <circuit.json> <input>         - Single input circuit (NOT)");
        println!(
            "  cargo run -- <circuit.json> <input1> <input2> - Two input circuit (AND/OR/XOR)"
        );
        println!("Examples:");
        println!("  cargo run -- circuits/not.json 1");
        println!("  cargo run -- circuits/and.json 1 0");
    }

    Ok(())
}

fn run_single_input_circuit(circuit_file: &str, input: bool) -> Result<()> {
    let circuit = Circuit::from_file(circuit_file)?;

    // Create secret shares
    let (alice_share, bob_share) = secret_share(input);

    // Set up input shares
    let mut alice_shares = HashMap::new();
    let mut bob_shares = HashMap::new();

    alice_shares.insert(1, alice_share); // wire 1: input
    bob_shares.insert(1, bob_share);

    // Evaluate circuit
    let (alice_output, bob_output) =
        evaluate_circuit_two_party(&circuit, &alice_shares, &bob_shares)?;

    // Reconstruct result
    let result = reconstruct_shares(alice_output, bob_output);

    println!("Input: {input} -> Output: {result}");

    // Calculate expected result
    let expected = match circuit.name.as_str() {
        "NOT_gate" => !input,
        _ => !input, // Default for single input
    };
    assert_eq!(result, expected);

    Ok(())
}

fn run_two_input_circuit(circuit_file: &str, input1: bool, input2: bool) -> Result<()> {
    let circuit = Circuit::from_file(circuit_file)?;

    // Create secret shares
    let (alice_share1, bob_share1) = secret_share(input1);
    let (alice_share2, bob_share2) = secret_share(input2);

    // Set up input shares
    let mut alice_shares = HashMap::new();
    let mut bob_shares = HashMap::new();

    alice_shares.insert(1, alice_share1); // wire 1: input1
    alice_shares.insert(2, alice_share2); // wire 2: input2
    bob_shares.insert(1, bob_share1);
    bob_shares.insert(2, bob_share2);

    // Evaluate circuit
    let (alice_output, bob_output) =
        evaluate_circuit_two_party(&circuit, &alice_shares, &bob_shares)?;

    // Reconstruct result
    let result = reconstruct_shares(alice_output, bob_output);

    println!("Inputs: {input1} & {input2} -> Output: {result}");

    // Calculate expected result
    let expected = match circuit.name.as_str() {
        "AND_gate" => input1 & input2,
        "OR_gate" => input1 | input2,
        "XOR_gate" => input1 ^ input2,
        _ => input1 & input2, // Default
    };
    assert_eq!(result, expected);

    Ok(())
}
