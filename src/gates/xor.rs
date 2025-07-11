use anyhow::Result;

/// Compute XOR gate for n parties
/// XOR is linear in GF(2), so each party simply XORs their shares locally
pub fn xor_gate(party_shares: &[(bool, bool)]) -> Result<Vec<bool>> {
    let n = party_shares.len();

    if n < 2 {
        return Err(anyhow::anyhow!("Need at least 2 parties for XOR gate"));
    }

    // Each party computes xi ⊕ yi locally
    let result_shares: Vec<bool> = party_shares.iter().map(|(xi, yi)| *xi ^ *yi).collect();

    Ok(result_shares)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_gate_2_party() {
        let shares = vec![(true, false), (false, true)];
        let result = xor_gate(&shares).unwrap();

        // Reconstruct: result[0] ⊕ result[1]
        let reconstructed = result[0] ^ result[1];

        // Expected: (true ⊕ false) ⊕ (false ⊕ true) = true ⊕ true = false
        assert_eq!(reconstructed, false);
    }

    #[test]
    fn test_xor_gate_3_party() {
        let shares = vec![(true, false), (false, true), (true, true)];
        let result = xor_gate(&shares).unwrap();

        // Reconstruct: result[0] ⊕ result[1] ⊕ result[2]
        let reconstructed = result[0] ^ result[1] ^ result[2];

        // Expected: (true ⊕ false) ⊕ (false ⊕ true) ⊕ (true ⊕ true) = true ⊕ true ⊕ false = false
        assert_eq!(reconstructed, false);
    }

    #[test]
    fn test_xor_gate_4_party() {
        let shares = vec![(true, true), (false, false), (true, false), (false, true)];
        let result = xor_gate(&shares).unwrap();

        // Reconstruct all shares
        let reconstructed = result.iter().fold(false, |acc, &x| acc ^ x);

        // Expected: (true ⊕ true) ⊕ (false ⊕ false) ⊕ (true ⊕ false) ⊕ (false ⊕ true)
        //         = false ⊕ false ⊕ true ⊕ true = false
        assert_eq!(reconstructed, false);
    }
}
