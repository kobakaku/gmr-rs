use anyhow::Result;
use std::collections::HashMap;

use crate::circuit::{Circuit, GateType};
use crate::gates::{
    and_gate, and_gate_3party_local, not_gate, not_gate_3party, or_gate, xor_gate, xor_gate_3party,
};

/// Type alias for three-party computation results
type ThreePartyShares = (HashMap<u32, bool>, HashMap<u32, bool>, HashMap<u32, bool>);

/// Party shares for multi-party computation
#[derive(Debug, Clone)]
pub enum PartyShares {
    TwoParty {
        alice: HashMap<u32, bool>,
        bob: HashMap<u32, bool>,
    },
    ThreeParty {
        party0: HashMap<u32, bool>,
        party1: HashMap<u32, bool>,
        party2: HashMap<u32, bool>,
    },
}

/// Evaluate a complete circuit with multi-party support
pub fn execute_circuit(circuit: &Circuit, shares: PartyShares) -> Result<PartyShares> {
    match shares {
        PartyShares::TwoParty { alice, bob } => {
            let (alice_result, bob_result) = execute_circuit_2party(circuit, &alice, &bob)?;
            Ok(PartyShares::TwoParty {
                alice: alice_result,
                bob: bob_result,
            })
        }
        PartyShares::ThreeParty {
            party0,
            party1,
            party2,
        } => {
            let (p0_result, p1_result, p2_result) =
                execute_circuit_3party(circuit, &party0, &party1, &party2)?;
            Ok(PartyShares::ThreeParty {
                party0: p0_result,
                party1: p1_result,
                party2: p2_result,
            })
        }
    }
}

/// Evaluate a complete circuit with two parties and return all intermediate shares
pub fn execute_circuit_2party(
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

/// Evaluate a complete circuit with three parties and return all intermediate shares
pub fn execute_circuit_3party(
    circuit: &Circuit,
    party0_shares: &HashMap<u32, bool>,
    party1_shares: &HashMap<u32, bool>,
    party2_shares: &HashMap<u32, bool>,
) -> Result<ThreePartyShares> {
    let mut p0_output_shares = party0_shares.clone();
    let mut p1_output_shares = party1_shares.clone();
    let mut p2_output_shares = party2_shares.clone();

    for gate in &circuit.gates {
        match gate.gate_type {
            GateType::XOR => {
                let p0_a = p0_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 0 input A"))?;
                let p0_b = p0_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 0 input B"))?;
                let p1_a = p1_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 1 input A"))?;
                let p1_b = p1_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 1 input B"))?;
                let p2_a = p2_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 2 input A"))?;
                let p2_b = p2_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 2 input B"))?;

                let p0_result = xor_gate_3party(p0_a, p0_b);
                let p1_result = xor_gate_3party(p1_a, p1_b);
                let p2_result = xor_gate_3party(p2_a, p2_b);

                p0_output_shares.insert(gate.id, p0_result);
                p1_output_shares.insert(gate.id, p1_result);
                p2_output_shares.insert(gate.id, p2_result);
            }
            GateType::NOT => {
                let p0_input = p0_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 0 input"))?;
                let p1_input = p1_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 1 input"))?;
                let p2_input = p2_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 2 input"))?;

                let p0_result = not_gate_3party(0, p0_input);
                let p1_result = not_gate_3party(1, p1_input);
                let p2_result = not_gate_3party(2, p2_input);

                p0_output_shares.insert(gate.id, p0_result);
                p1_output_shares.insert(gate.id, p1_result);
                p2_output_shares.insert(gate.id, p2_result);
            }
            GateType::AND => {
                let p0_a = p0_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 0 input A"))?;
                let p0_b = p0_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 0 input B"))?;
                let p1_a = p1_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 1 input A"))?;
                let p1_b = p1_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 1 input B"))?;
                let p2_a = p2_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 2 input A"))?;
                let p2_b = p2_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 2 input B"))?;

                let party_shares = [(p0_a, p0_b), (p1_a, p1_b), (p2_a, p2_b)];
                let result_shares = and_gate_3party_local(party_shares)?;

                p0_output_shares.insert(gate.id, result_shares[0]);
                p1_output_shares.insert(gate.id, result_shares[1]);
                p2_output_shares.insert(gate.id, result_shares[2]);
            }
            GateType::OR => {
                // OR gate using De Morgan's law: A OR B = NOT(NOT(A) AND NOT(B))
                // For now, we'll implement this as a combination of NOT and AND gates
                let p0_a = p0_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 0 input A"))?;
                let p0_b = p0_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 0 input B"))?;
                let p1_a = p1_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 1 input A"))?;
                let p1_b = p1_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 1 input B"))?;
                let p2_a = p2_output_shares
                    .get(&gate.inputs[0])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 2 input A"))?;
                let p2_b = p2_output_shares
                    .get(&gate.inputs[1])
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Missing Party 2 input B"))?;

                // NOT(A)
                let not_a_shares = [
                    not_gate_3party(0, p0_a),
                    not_gate_3party(1, p1_a),
                    not_gate_3party(2, p2_a),
                ];

                // NOT(B)
                let not_b_shares = [
                    not_gate_3party(0, p0_b),
                    not_gate_3party(1, p1_b),
                    not_gate_3party(2, p2_b),
                ];

                // NOT(A) AND NOT(B)
                let and_shares = and_gate_3party_local([
                    (not_a_shares[0], not_b_shares[0]),
                    (not_a_shares[1], not_b_shares[1]),
                    (not_a_shares[2], not_b_shares[2]),
                ])?;

                // NOT(NOT(A) AND NOT(B)) = A OR B
                let p0_result = not_gate_3party(0, and_shares[0]);
                let p1_result = not_gate_3party(1, and_shares[1]);
                let p2_result = not_gate_3party(2, and_shares[2]);

                p0_output_shares.insert(gate.id, p0_result);
                p1_output_shares.insert(gate.id, p1_result);
                p2_output_shares.insert(gate.id, p2_result);
            }
        }
    }

    Ok((p0_output_shares, p1_output_shares, p2_output_shares))
}

/// Create secret shares for 2-party computation
/// The secret value is split as: value = share0 ⊕ share1
pub fn secret_share(value: bool) -> (bool, bool) {
    let share0 = rand::random::<bool>();
    let share1 = value ^ share0;
    (share0, share1)
}

/// Reconstruct secret from 2 shares
pub fn reconstruct_shares(share0: bool, share1: bool) -> bool {
    share0 ^ share1
}

/// Create secret shares for 3-party computation
/// The secret value is split as: value = share0 ⊕ share1 ⊕ share2
pub fn secret_share_3party(value: bool) -> (bool, bool, bool) {
    let share0 = rand::random::<bool>();
    let share1 = rand::random::<bool>();
    let share2 = value ^ share0 ^ share1;
    (share0, share1, share2)
}

/// Reconstruct secret from 3 shares
pub fn reconstruct_shares_3party(share0: bool, share1: bool, share2: bool) -> bool {
    share0 ^ share1 ^ share2
}
