use crate::circuit::types::{Circuit, CircuitFile};
use serde_json;
use std::fs;
use std::path::Path;

pub fn load_circuit_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<CircuitFile, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let circuit_file: CircuitFile = serde_json::from_str(&contents)?;
    Ok(circuit_file)
}

pub fn get_circuit_by_id<'a>(circuit_file: &'a CircuitFile, id: &'a str) -> Option<&'a Circuit> {
    circuit_file.circuits.iter().find(|c| c.id == id)
}
