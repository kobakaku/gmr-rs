use crate::circuit::{Circuit, GateType, WireId};
use anyhow::Result;
use std::collections::HashMap;

/// Evaluate a circuit locally (without secret sharing) for verification purposes
pub struct LocalEvaluator;

impl LocalEvaluator {
    /// Evaluate a circuit with given inputs and return all gate outputs
    pub fn evaluate(circuit: &Circuit, inputs: &[bool]) -> Result<HashMap<WireId, bool>> {
        let mut wire_values = HashMap::new();

        // Initialize input wires
        for (i, &input) in inputs.iter().enumerate() {
            let wire_id = circuit.metadata.inputs[i].id;
            wire_values.insert(wire_id, input);
        }

        // Evaluate each gate in order
        for gate in &circuit.gates {
            let result = match gate.gate_type {
                GateType::AND => {
                    let a = Self::get_wire_value(&wire_values, gate.inputs[0])?;
                    let b = Self::get_wire_value(&wire_values, gate.inputs[1])?;
                    a & b
                }
                GateType::OR => {
                    let a = Self::get_wire_value(&wire_values, gate.inputs[0])?;
                    let b = Self::get_wire_value(&wire_values, gate.inputs[1])?;
                    a | b
                }
                GateType::XOR => {
                    let a = Self::get_wire_value(&wire_values, gate.inputs[0])?;
                    let b = Self::get_wire_value(&wire_values, gate.inputs[1])?;
                    a ^ b
                }
                GateType::NOT => {
                    let a = Self::get_wire_value(&wire_values, gate.inputs[0])?;
                    !a
                }
            };

            wire_values.insert(gate.id, result);
        }

        Ok(wire_values)
    }

    /// Get the output value for a specific gate
    pub fn get_output(circuit: &Circuit, inputs: &[bool], wire_id: WireId) -> Result<bool> {
        let wire_values = Self::evaluate(circuit, inputs)?;
        wire_values
            .get(&wire_id)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Wire {} not found in circuit", wire_id))
    }

    /// Helper to get wire value with error handling
    fn get_wire_value(wire_values: &HashMap<WireId, bool>, wire_id: WireId) -> Result<bool> {
        wire_values
            .get(&wire_id)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Wire {} not found", wire_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::{Circuit, CircuitMetadata, Gate, GateType, InputInfo, OutputInfo};

    #[test]
    fn test_local_evaluator_and_gate() {
        let circuit = Circuit {
            name: "test_and".to_string(),
            description: "Test AND gate".to_string(),
            gates: vec![Gate {
                id: 3,
                gate_type: GateType::AND,
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

        // Test all combinations
        assert_eq!(
            LocalEvaluator::get_output(&circuit, &[false, false], 3).unwrap(),
            false
        );
        assert_eq!(
            LocalEvaluator::get_output(&circuit, &[false, true], 3).unwrap(),
            false
        );
        assert_eq!(
            LocalEvaluator::get_output(&circuit, &[true, false], 3).unwrap(),
            false
        );
        assert_eq!(
            LocalEvaluator::get_output(&circuit, &[true, true], 3).unwrap(),
            true
        );
    }
}
