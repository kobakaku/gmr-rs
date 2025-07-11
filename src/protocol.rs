use anyhow::Result;
use std::collections::HashMap;

use crate::circuit::{Circuit, GateType, WireId};
use crate::gates::{and_gate, not_gate, or_gate, xor_gate};

/// Party shares for multi-party computation
pub type PartyShares = Vec<HashMap<WireId, bool>>;

/// GMW Protocol implementation for secure multi-party computation
pub struct GmwProtocol {
    party_count: usize,
}

impl GmwProtocol {
    /// Create a new GMW protocol instance for n parties
    pub fn new(party_count: usize) -> Result<Self> {
        if party_count < 2 {
            return Err(anyhow::anyhow!("Need at least 2 parties for computation"));
        }

        Ok(Self { party_count })
    }

    /// Create secret shares for n-party computation
    /// The secret value is split as: value = share0 ⊕ share1 ⊕ ... ⊕ share(n-1)
    pub fn secret_share(&self, value: bool) -> Vec<bool> {
        let mut shares = Vec::with_capacity(self.party_count);
        let mut accumulated_xor = value;

        // Generate n-1 random shares
        for _ in 0..self.party_count - 1 {
            let share = rand::random::<bool>();
            shares.push(share);
            accumulated_xor ^= share;
        }

        // Last share ensures XOR of all shares equals the value
        shares.push(accumulated_xor);

        shares
    }

    /// Reconstruct secret from n shares
    pub fn reconstruct_shares(&self, shares: &[bool]) -> bool {
        shares.iter().fold(false, |acc, &share| acc ^ share)
    }

    /// Evaluate a complete circuit with multi-party support
    pub fn execute_circuit(&self, circuit: &Circuit, shares: PartyShares) -> Result<PartyShares> {
        if shares.len() != self.party_count {
            return Err(anyhow::anyhow!(
                "Party count mismatch: expected {}, got {}",
                self.party_count,
                shares.len()
            ));
        }

        let mut output_shares: Vec<HashMap<WireId, bool>> = shares.clone();

        for gate in &circuit.gates {
            let result_shares = match gate.gate_type {
                GateType::XOR | GateType::AND | GateType::OR => {
                    // Binary gates: collect two inputs from each party
                    let party_inputs = self.collect_binary_inputs(&output_shares, &gate.inputs)?;

                    match gate.gate_type {
                        GateType::XOR => xor_gate(&party_inputs)?,
                        GateType::AND => and_gate(&party_inputs)?,
                        GateType::OR => or_gate(&party_inputs)?,
                        _ => unreachable!(),
                    }
                }
                GateType::NOT => {
                    // Unary gate: collect one input from each party
                    let party_inputs = self.collect_unary_inputs(&output_shares, gate.inputs[0])?;
                    not_gate(&party_inputs)?
                }
            };

            // Store results for all parties
            for (party_id, result) in result_shares.into_iter().enumerate() {
                output_shares[party_id].insert(gate.id, result);
            }
        }

        Ok(output_shares)
    }

    /// Create party shares from inputs and run circuit with n parties
    pub fn run_circuit(&self, circuit: &Circuit, inputs: &[bool]) -> Result<Vec<(String, bool)>> {
        if circuit.metadata.outputs.is_empty() {
            return Err(anyhow::anyhow!(
                "Circuit has no output metadata. Please add metadata to the circuit JSON file."
            ));
        }

        let expected_inputs = circuit.metadata.inputs.len();
        if expected_inputs > 0 && inputs.len() != expected_inputs {
            return Err(anyhow::anyhow!(
                "Circuit expects {} inputs but got {}",
                expected_inputs,
                inputs.len()
            ));
        }

        // Create n-party secret shares
        let mut party_shares: Vec<HashMap<WireId, bool>> = vec![HashMap::new(); self.party_count];

        for (i, &input) in inputs.iter().enumerate() {
            let shares = self.secret_share(input);
            let wire_id = circuit.metadata.inputs[i].id;

            for (party_id, share) in shares.into_iter().enumerate() {
                party_shares[party_id].insert(wire_id, share);
            }
        }

        // Execute circuit
        let result_shares = self.execute_circuit(circuit, party_shares)?;

        // Collect outputs
        let mut outputs = Vec::new();
        for output_info in &circuit.metadata.outputs {
            let output_shares: Vec<bool> = result_shares
                .iter()
                .map(|party| {
                    party
                        .get(&output_info.id)
                        .copied()
                        .ok_or_else(|| anyhow::anyhow!("Missing output gate {}", output_info.id))
                })
                .collect::<Result<Vec<_>>>()?;
            let result = self.reconstruct_shares(&output_shares);
            outputs.push((output_info.name.clone(), result));
        }

        Ok(outputs)
    }

    /// Collect binary inputs (two inputs per party) for gates like XOR, AND, OR
    fn collect_binary_inputs(
        &self,
        output_shares: &[HashMap<WireId, bool>],
        gate_inputs: &[WireId],
    ) -> Result<Vec<(bool, bool)>> {
        let mut party_inputs = Vec::with_capacity(self.party_count);

        for (party_id, party_share) in output_shares.iter().enumerate().take(self.party_count) {
            let input_a = party_share
                .get(&gate_inputs[0])
                .copied()
                .ok_or_else(|| anyhow::anyhow!("Missing Party {} input A", party_id))?;
            let input_b = party_share
                .get(&gate_inputs[1])
                .copied()
                .ok_or_else(|| anyhow::anyhow!("Missing Party {} input B", party_id))?;
            party_inputs.push((input_a, input_b));
        }

        Ok(party_inputs)
    }

    /// Collect unary inputs (one input per party) for gates like NOT
    fn collect_unary_inputs(
        &self,
        output_shares: &[HashMap<WireId, bool>],
        input_wire: WireId,
    ) -> Result<Vec<bool>> {
        let mut party_inputs = Vec::with_capacity(self.party_count);

        for (party_id, party_share) in output_shares.iter().enumerate().take(self.party_count) {
            let input = party_share
                .get(&input_wire)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("Missing Party {} input", party_id))?;
            party_inputs.push(input);
        }

        Ok(party_inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_share() {
        // Test with different party counts
        for n in 2..=6 {
            let protocol = GmwProtocol::new(n).unwrap();

            // Test true value
            let shares = protocol.secret_share(true);
            assert_eq!(shares.len(), n);
            assert_eq!(protocol.reconstruct_shares(&shares), true);

            // Test false value
            let shares = protocol.secret_share(false);
            assert_eq!(shares.len(), n);
            assert_eq!(protocol.reconstruct_shares(&shares), false);
        }
    }

    #[test]
    fn test_execute_circuit_xor() {
        use crate::circuit::{Circuit, CircuitMetadata, Gate, GateType, InputInfo, OutputInfo};

        let circuit = Circuit {
            name: "test_xor".to_string(),
            description: "Test XOR gate".to_string(),
            gates: vec![Gate {
                id: 3,
                gate_type: GateType::XOR,
                inputs: vec![1, 2],
            }],
            metadata: CircuitMetadata {
                inputs: vec![
                    InputInfo {
                        name: "a".to_string(),
                        id: 1,
                    },
                    InputInfo {
                        name: "b".to_string(),
                        id: 2,
                    },
                ],
                outputs: vec![OutputInfo {
                    name: "result".to_string(),
                    id: 3,
                }],
            },
        };

        // Test with 4 parties: true XOR false = true
        let protocol = GmwProtocol::new(4).unwrap();
        let a_shares = protocol.secret_share(true);
        let b_shares = protocol.secret_share(false);

        let mut party_shares = Vec::new();
        for i in 0..4 {
            let mut shares = HashMap::new();
            shares.insert(1, a_shares[i]);
            shares.insert(2, b_shares[i]);
            party_shares.push(shares);
        }

        let result = protocol.execute_circuit(&circuit, party_shares).unwrap();

        // Collect output shares and reconstruct
        let output_shares: Vec<bool> = result
            .iter()
            .map(|party| party.get(&3).copied().unwrap())
            .collect();

        assert_eq!(protocol.reconstruct_shares(&output_shares), true);
    }
}
