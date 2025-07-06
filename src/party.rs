use crate::circuit::{Circuit, GateType};
use crate::gates::{and_gate, not_gate, or_gate, xor_gate};
use anyhow::Result;
use std::collections::HashMap;

/// Simplified party that handles circuit evaluation without complex communication
#[derive(Debug)]
pub struct SimpleParty {
    pub id: u8,
    pub shares: HashMap<u32, bool>,
}

impl SimpleParty {
    pub fn new(id: u8) -> Self {
        SimpleParty {
            id,
            shares: HashMap::new(),
        }
    }

    pub fn set_share(&mut self, wire_id: u32, value: bool) {
        self.shares.insert(wire_id, value);
    }

    pub fn get_share(&self, wire_id: u32) -> Option<bool> {
        self.shares.get(&wire_id).copied()
    }
}

/// Evaluate a complete circuit with two parties
pub fn evaluate_circuit_two_party(
    circuit: &Circuit,
    alice_shares: &HashMap<u32, bool>,
    bob_shares: &HashMap<u32, bool>,
) -> Result<(bool, bool)> {
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

                // XOR is local
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

                // NOT is local (only one party flips the bit)
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

                // AND requires OT
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

                // OR requires OT (using De Morgan's law internally)
                let (alice_result, bob_result) = or_gate(alice_a, alice_b, bob_a, bob_b)?;

                alice_output_shares.insert(gate.id, alice_result);
                bob_output_shares.insert(gate.id, bob_result);
            }
        }
    }

    // Return the final output shares (assuming output wire is the last gate)
    let output_wire = circuit
        .gates
        .last()
        .ok_or_else(|| anyhow::anyhow!("Empty circuit"))?
        .id;

    let alice_final = alice_output_shares
        .get(&output_wire)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Missing Alice final output"))?;
    let bob_final = bob_output_shares
        .get(&output_wire)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Missing Bob final output"))?;

    Ok((alice_final, bob_final))
}

pub fn secret_share(value: bool) -> (bool, bool) {
    let share1 = rand::random::<bool>();
    let share2 = value ^ share1;
    (share1, share2)
}

pub fn reconstruct_shares(share1: bool, share2: bool) -> bool {
    share1 ^ share2
}
