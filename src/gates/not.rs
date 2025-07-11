use anyhow::Result;

/// Compute NOT gate for n parties
/// Only the first party flips their share, others keep their shares unchanged
/// This ensures that when reconstructed: share[0] ⊕ share[1] ⊕ ... = !original_value
pub fn not_gate(party_shares: &[bool]) -> Result<Vec<bool>> {
    let n = party_shares.len();

    if n < 2 {
        return Err(anyhow::anyhow!("Need at least 2 parties for NOT gate"));
    }

    let mut result_shares = party_shares.to_vec();

    // Only party 0 flips their share
    result_shares[0] = !result_shares[0];

    Ok(result_shares)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_gate_2_party() {
        let shares = vec![true, false];
        let result = not_gate(&shares).unwrap();

        // Reconstruct: result[0] ⊕ result[1]
        let reconstructed = result[0] ^ result[1];

        // Original: true ⊕ false = true
        // Expected: !true = false
        assert_eq!(reconstructed, false);
    }

    #[test]
    fn test_not_gate_3_party() {
        let shares = vec![true, false, true];
        let result = not_gate(&shares).unwrap();

        // Reconstruct: result[0] ⊕ result[1] ⊕ result[2]
        let reconstructed = result[0] ^ result[1] ^ result[2];

        // Original: true ⊕ false ⊕ true = false
        // Expected: !false = true
        assert_eq!(reconstructed, true);
    }

    #[test]
    fn test_not_gate_4_party() {
        let shares = vec![false, true, false, true];
        let result = not_gate(&shares).unwrap();

        // Reconstruct all shares
        let reconstructed = result.iter().fold(false, |acc, &x| acc ^ x);

        // Original: false ⊕ true ⊕ false ⊕ true = false
        // Expected: !false = true
        assert_eq!(reconstructed, true);
    }
}
