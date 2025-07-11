use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

pub type WireId = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub name: String,
    pub description: String,
    pub gates: Vec<Gate>,
    pub metadata: CircuitMetadata,
}

impl Circuit {
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let circuit: Circuit = serde_json::from_str(&contents)?;
        Ok(circuit)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        let circuit: Circuit = serde_json::from_str(json)?;
        Ok(circuit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gate {
    pub id: WireId,
    #[serde(rename = "type")]
    pub gate_type: GateType,
    #[serde(rename = "in")]
    pub inputs: Vec<WireId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateType {
    XOR,
    NOT,
    AND,
    OR,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CircuitMetadata {
    pub inputs: Vec<InputInfo>,
    pub outputs: Vec<OutputInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputInfo {
    pub name: String,
    pub id: WireId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputInfo {
    pub name: String,
    pub id: WireId,
}
