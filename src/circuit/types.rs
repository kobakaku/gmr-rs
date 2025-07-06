use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub gates: Vec<Gate>,
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
    AND,
    OR,
}
