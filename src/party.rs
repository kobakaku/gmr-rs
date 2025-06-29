use crate::circuit::{Circuit, Gate, GateType};
use crate::gates::{not_gate, xor_gate};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Party {
    pub id: u8,
    pub shares: HashMap<u32, bool>,
}

impl Party {
    pub fn new(id: u8) -> Self {
        Party {
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

    pub fn evaluate_gate(&mut self, gate: &Gate) {
        let result = match gate.gate_type {
            GateType::XOR => {
                let input_a = self.get_share(gate.inputs[0]).expect("Missing input share");
                let input_b = self.get_share(gate.inputs[1]).expect("Missing input share");
                xor_gate(self.id, input_a, input_b)
            }
            GateType::NOT => {
                let input = self.get_share(gate.inputs[0]).expect("Missing input share");
                not_gate(self.id, input)
            }
        };
        self.set_share(gate.id, result);
    }

    pub fn evaluate_circuit(&mut self, circuit: &Circuit) {
        for gate in &circuit.gates {
            self.evaluate_gate(gate);
        }
    }
}

pub fn secret_share(value: bool) -> (bool, bool) {
    let share1 = rand::random::<bool>();
    let share2 = value ^ share1;
    (share1, share2)
}

pub fn reconstruct_shares(share1: bool, share2: bool) -> bool {
    share1 ^ share2
}
