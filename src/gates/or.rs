use crate::gates::and::and_gate;
use crate::gates::not::not_gate;
use anyhow::Result;

/// Compute OR gate for n parties using De Morgan's law: x | y = ~(~x & ~y)
/// 1. NOT both inputs
/// 2. AND the results
/// 3. NOT the final result
pub fn or_gate(party_shares: &[(bool, bool)]) -> Result<Vec<bool>> {
    let n = party_shares.len();

    if n < 2 {
        return Err(anyhow::anyhow!("Need at least 2 parties for OR gate"));
    }

    // Step 1: Apply NOT to both inputs (xi, yi) -> (~xi, ~yi)
    let mut not_x_shares = Vec::with_capacity(n);
    let mut not_y_shares = Vec::with_capacity(n);

    for (xi, yi) in party_shares {
        not_x_shares.push(*xi);
        not_y_shares.push(*yi);
    }

    // NOT the x shares
    let not_x = not_gate(&not_x_shares)?;

    // NOT the y shares
    let not_y = not_gate(&not_y_shares)?;

    // Step 2: AND the NOT results: ~x & ~y
    let not_x_and_not_y_shares: Vec<(bool, bool)> = not_x.into_iter().zip(not_y).collect();

    let and_result = and_gate(&not_x_and_not_y_shares)?;

    // Step 3: NOT the final result: ~(~x & ~y) = x | y
    let or_result = not_gate(&and_result)?;

    Ok(or_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or_gate_2_party() {
        let shares = vec![(true, false), (false, false)];
        let result = or_gate(&shares).unwrap();

        // Reconstruct: result[0] ⊕ result[1]
        let reconstructed = result[0] ^ result[1];

        // Original: (true | false) | (false | false) = true | false = true
        assert_eq!(reconstructed, true);
    }

    #[test]
    fn test_or_gate_3_party() {
        let shares = vec![(false, false), (false, false), (false, false)];
        let result = or_gate(&shares).unwrap();

        // Reconstruct: result[0] ⊕ result[1] ⊕ result[2]
        let reconstructed = result[0] ^ result[1] ^ result[2];

        // Original: (false | false) | (false | false) | (false | false) = false
        assert_eq!(reconstructed, false);
    }

    #[test]
    fn test_or_gate_4_party_all_true() {
        let shares = vec![(true, true), (true, true), (true, true), (true, true)];

        // x = true ⊕ true ⊕ true ⊕ true = false
        // y = true ⊕ true ⊕ true ⊕ true = false
        // Expected: false | false = false

        let result = or_gate(&shares).unwrap();

        // Reconstruct all shares
        let reconstructed = result.iter().fold(false, |acc, &x| acc ^ x);

        assert_eq!(reconstructed, false);
    }
}
