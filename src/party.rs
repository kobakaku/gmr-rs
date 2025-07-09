use anyhow::Result;
use std::collections::HashMap;

use crate::circuit::{Circuit, GateType};
use crate::gates::{and_gate, not_gate, or_gate, xor_gate};

/// Evaluate a complete circuit with two parties and return all intermediate shares
pub fn execute_circuit(
    circuit: &Circuit,
    alice_shares: &HashMap<u32, bool>,
    bob_shares: &HashMap<u32, bool>,
) -> Result<(HashMap<u32, bool>, HashMap<u32, bool>)> {
    let mut alice_output_shares = alice_shares.clone();
    let mut bob_output_shares = bob_shares.clone();

    for gate in &circuit.gates {
        match gate.gate_type {
            GateType::XOR => {
                let alice_a = alice_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Alice input A"))?;
                let alice_b = alice_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Alice input B"))?;
                let bob_a = bob_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Bob input A"))?;
                let bob_b = bob_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Bob input B"))?;

                let alice_result = xor_gate(alice_a, alice_b);
                let bob_result = xor_gate(bob_a, bob_b);

                alice_output_shares.insert(gate.id, alice_result);
                bob_output_shares.insert(gate.id, bob_result);
            }
            GateType::NOT => {
                let alice_input = alice_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Alice input"))?;
                let bob_input = bob_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Bob input"))?;

                let alice_result = not_gate(0, alice_input);
                let bob_result = not_gate(1, bob_input);

                alice_output_shares.insert(gate.id, alice_result);
                bob_output_shares.insert(gate.id, bob_result);
            }
            GateType::AND => {
                let alice_a = alice_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Alice input A"))?;
                let alice_b = alice_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Alice input B"))?;
                let bob_a = bob_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Bob input A"))?;
                let bob_b = bob_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Bob input B"))?;

                let (alice_result, bob_result) = and_gate(alice_a, alice_b, bob_a, bob_b)?;

                alice_output_shares.insert(gate.id, alice_result);
                bob_output_shares.insert(gate.id, bob_result);
            }
            GateType::OR => {
                let alice_a = alice_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Alice input A"))?;
                let alice_b = alice_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Alice input B"))?;
                let bob_a = bob_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Bob input A"))?;
                let bob_b = bob_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Bob input B"))?;

                let (alice_result, bob_result) = or_gate(alice_a, alice_b, bob_a, bob_b)?;

                alice_output_shares.insert(gate.id, alice_result);
                bob_output_shares.insert(gate.id, bob_result);
            }
        }
    }

    Ok((alice_output_shares, bob_output_shares))
}

pub fn secret_share(value: bool) -> (bool, bool) {
    let share1 = rand::random::<bool>();
    let share2 = value ^ share1;
    (share1, share2)
}

pub fn reconstruct_shares(share1: bool, share2: bool) -> bool {
    share1 ^ share2
}
