use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitFile {
    pub name: String,
    pub circuits: Vec<Circuit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub id: String,
    pub alice: Vec<u32>,
    #[serde(default)]
    pub bob: Vec<u32>,
    pub out: Vec<u32>,
    pub gates: Vec<Gate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gate {
    pub id: u32,
    #[serde(rename = "type")]
    pub gate_type: GateType,
    #[serde(rename = "in")]
    pub inputs: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateType {
    XOR,
    NOT,
}

impl GateType {
    pub fn num_inputs(&self) -> usize {
        match self {
            GateType::XOR => 2,
            GateType::NOT => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvaluatedGate {
    pub id: u32,
    pub value: bool,
}
