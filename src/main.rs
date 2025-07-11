use anyhow::Result;
use std::collections::HashMap;
use std::env;

use gmw_rs::{
    execute_circuit, reconstruct_shares, reconstruct_shares_3party, secret_share,
    secret_share_3party, Circuit, LocalEvaluator, PartyShares,
};

/// Run a circuit with unified interface
fn run_circuit(circuit_file: &str, inputs: Vec<bool>, use_3party: bool) -> Result<()> {
    let circuit = Circuit::from_file(circuit_file)?;

    // Validate input count
    if circuit.metadata.input_count > 0 && inputs.len() != circuit.metadata.input_count {
        return Err(anyhow::anyhow!(
            "Circuit expects {} inputs but got {}",
            circuit.metadata.input_count,
            inputs.len()
        ));
    }

    // Create shares and evaluate circuit based on party count
    let result_shares = if use_3party {
        // Create 3-party secret shares
        let mut party0_shares = HashMap::new();
        let mut party1_shares = HashMap::new();
        let mut party2_shares = HashMap::new();

        for (i, &input) in inputs.iter().enumerate() {
            let (share0, share1, share2) = secret_share_3party(input);
            let wire_id = (i + 1) as u32;
            party0_shares.insert(wire_id, share0);
            party1_shares.insert(wire_id, share1);
            party2_shares.insert(wire_id, share2);
        }

        let shares = PartyShares::ThreeParty {
            party0: party0_shares,
            party1: party1_shares,
            party2: party2_shares,
        };
        execute_circuit(&circuit, shares)?
    } else {
        // Create 2-party secret shares
        let mut alice_shares = HashMap::new();
        let mut bob_shares = HashMap::new();

        for (i, &input) in inputs.iter().enumerate() {
            let (alice_share, bob_share) = secret_share(input);
            let wire_id = (i + 1) as u32;
            alice_shares.insert(wire_id, alice_share);
            bob_shares.insert(wire_id, bob_share);
        }

        let shares = PartyShares::TwoParty {
            alice: alice_shares,
            bob: bob_shares,
        };
        execute_circuit(&circuit, shares)?
    };

    println!("Inputs: {inputs:?}");

    // Handle outputs using metadata
    if circuit.metadata.outputs.is_empty() {
        return Err(anyhow::anyhow!(
            "Circuit has no output metadata. Please add metadata to the circuit JSON file."
        ));
    }

    println!("Outputs:");
    for output_info in &circuit.metadata.outputs {
        let result = match &result_shares {
            PartyShares::TwoParty { alice, bob } => {
                let alice_output = alice.get(&output_info.gate_id).copied().ok_or_else(|| {
                    anyhow::anyhow!("Missing output gate {}", output_info.gate_id)
                })?;
                let bob_output = bob.get(&output_info.gate_id).copied().ok_or_else(|| {
                    anyhow::anyhow!("Missing output gate {}", output_info.gate_id)
                })?;
                reconstruct_shares(alice_output, bob_output)
            }
            PartyShares::ThreeParty {
                party0,
                party1,
                party2,
            } => {
                let party0_output = party0.get(&output_info.gate_id).copied().ok_or_else(|| {
                    anyhow::anyhow!("Missing output gate {}", output_info.gate_id)
                })?;
                let party1_output = party1.get(&output_info.gate_id).copied().ok_or_else(|| {
                    anyhow::anyhow!("Missing output gate {}", output_info.gate_id)
                })?;
                let party2_output = party2.get(&output_info.gate_id).copied().ok_or_else(|| {
                    anyhow::anyhow!("Missing output gate {}", output_info.gate_id)
                })?;
                reconstruct_shares_3party(party0_output, party1_output, party2_output)
            }
        };

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
    println!("Usage: cargo run -- [--3party] <circuit.json> <input1> [input2] [input3] ...");
    println!();
    println!("Options:");
    println!("  --3party    Use 3-party computation (local simulation)");
    println!();
    println!("Examples:");
    println!("  cargo run -- circuits/not.json 1");
    println!("  cargo run -- circuits/and.json 1 0");
    println!("  cargo run -- circuits/half_adder.json 1 1");
    println!("  cargo run -- --3party circuits/and.json 1 0");
    println!("  cargo run -- --3party circuits/xor.json 1 0");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    // Check for --3party flag
    let (use_3party, remaining_args) = if args.len() > 1 && args[1] == "--3party" {
        (true, &args[2..])
    } else {
        (false, &args[1..])
    };

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

    run_circuit(circuit_file, inputs, use_3party)
}
