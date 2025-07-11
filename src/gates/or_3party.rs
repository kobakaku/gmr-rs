use crate::gates::{and_gate_3party_local, not_gate_3party};
use anyhow::Result;

/// 3-party OR gate using De Morgan's law
/// x OR y = NOT(NOT(x) AND NOT(y))
pub fn or_gate_3party_local(
    _party_id: u32,
    party_shares: [(bool, bool); 3], // Each party's (x_share, y_share)
) -> Result<[bool; 3]> {
    // Apply NOT to each party's shares
    let not_x_shares: [bool; 3] = [
        not_gate_3party(0, party_shares[0].0),
        not_gate_3party(1, party_shares[1].0),
        not_gate_3party(2, party_shares[2].0),
    ];

    let not_y_shares: [bool; 3] = [
        not_gate_3party(0, party_shares[0].1),
        not_gate_3party(1, party_shares[1].1),
        not_gate_3party(2, party_shares[2].1),
    ];

    // Prepare shares for AND gate
    let and_input_shares = [
        (not_x_shares[0], not_y_shares[0]),
        (not_x_shares[1], not_y_shares[1]),
        (not_x_shares[2], not_y_shares[2]),
    ];

    // Compute NOT(x) AND NOT(y)
    let and_result = and_gate_3party_local(and_input_shares)?;

    // Apply final NOT
    let or_result = [
        not_gate_3party(0, and_result[0]),
        not_gate_3party(1, and_result[1]),
        not_gate_3party(2, and_result[2]),
    ];

    Ok(or_result)
}
