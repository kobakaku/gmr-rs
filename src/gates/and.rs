use crate::ot::BitOT;
use anyhow::Result;

/// Compute AND gate for n parties using GMW protocol
/// Each party has shares (xi, yi) and needs to compute xi & yi locally,
/// then use OT to compute cross terms xi*yj ⊕ xj*yi for all pairs i,j
pub fn and_gate(party_shares: &[(bool, bool)]) -> Result<Vec<bool>> {
    let n = party_shares.len();

    if n < 2 {
        return Err(anyhow::anyhow!("Need at least 2 parties for AND gate"));
    }

    // Step 1: Each party computes local term xi & yi
    let local_terms: Vec<bool> = party_shares.iter().map(|(xi, yi)| *xi & *yi).collect();

    // Step 2: Compute cross terms between all pairs of parties
    let mut cross_terms: Vec<Vec<bool>> = vec![vec![false; n]; n];

    for i in 0..n {
        for j in (i + 1)..n {
            let (xi, yi) = party_shares[i];
            let (xj, yj) = party_shares[j];

            // Compute cross term: xi*yj ⊕ xj*yi using OT
            let (cross_ij, cross_ji) = compute_cross_term_ot((xi, yi), (xj, yj))?;

            // Store cross terms for each party
            cross_terms[i][j] = cross_ij;
            cross_terms[j][i] = cross_ji;
        }
    }

    // Step 3: Each party combines local term with all cross terms
    let mut result_shares = Vec::with_capacity(n);
    for i in 0..n {
        let mut result = local_terms[i];

        // XOR all cross terms involving party i
        for j in 0..n {
            if i != j {
                result ^= cross_terms[i][j];
            }
        }

        result_shares.push(result);
    }

    Ok(result_shares)
}

/// Compute cross term between two parties using OT
/// Returns (share_for_party_i, share_for_party_j)
fn compute_cross_term_ot(
    party_i_shares: (bool, bool),
    party_j_shares: (bool, bool),
) -> Result<(bool, bool)> {
    let (xi, yi) = party_i_shares;
    let (xj, yj) = party_j_shares;

    // Party i acts as sender, party j as receiver
    // We need to compute xi·yj ⊕ xj·yi and split it into shares

    // Party i generates random bit ri (will be party i's share)
    let ri = rand::random::<bool>();

    // Party j needs to receive: (xi·yj ⊕ xj·yi) ⊕ ri
    // Using 1-out-of-4 OT based on (xj, yj) as choice bits

    // Party i prepares 4 messages for all possible (xj, yj) values:
    // (0,0): xi·0 ⊕ 0·yi ⊕ ri = 0 ⊕ ri = ri
    // (0,1): xi·1 ⊕ 0·yi ⊕ ri = xi ⊕ ri
    // (1,0): xi·0 ⊕ 1·yi ⊕ ri = yi ⊕ ri
    // (1,1): xi·1 ⊕ 1·yi ⊕ ri = xi ⊕ yi ⊕ ri
    let messages = (
        ri,           // (0,0)
        ri ^ xi,      // (0,1)
        ri ^ yi,      // (1,0)
        ri ^ xi ^ yi, // (1,1)
    );

    let choice = (xj, yj);
    let rj = BitOT::execute_1_out_of_4(messages, choice)?;

    Ok((ri, rj))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_gate_2_party() {
        // Test 2-party case (should match existing 2-party implementation)
        // Input: x=true, y=false
        // x shares: x0=true, x1=false (x0 ⊕ x1 = true)
        // y shares: y0=false, y1=true (y0 ⊕ y1 = true)
        // Wait, this doesn't make sense. Let me check the actual inputs

        // Actually, party_shares = [(x0, y0), (x1, y1)]
        // where x = x0 ⊕ x1 and y = y0 ⊕ y1
        // With shares = [(true, false), (false, true)]:
        // x = true ⊕ false = true
        // y = false ⊕ true = true
        // So we're computing true & true = true

        let shares = vec![(true, false), (false, true)];
        let result = and_gate(&shares).unwrap();

        // Reconstruct: result[0] ⊕ result[1] should equal true & true = true
        let reconstructed = result[0] ^ result[1];
        assert_eq!(reconstructed, true);
    }

    #[test]
    fn test_and_gate_3_party() {
        // Test 3-party case
        // party_shares = [(x0, y0), (x1, y1), (x2, y2)]
        // x = x0 ⊕ x1 ⊕ x2, y = y0 ⊕ y1 ⊕ y2
        let shares = vec![(true, true), (false, true), (true, false)];

        // x = true ⊕ false ⊕ true = false
        // y = true ⊕ true ⊕ false = false
        // Expected: false & false = false

        let result = and_gate(&shares).unwrap();

        // Reconstruct: result[0] ⊕ result[1] ⊕ result[2]
        let reconstructed = result[0] ^ result[1] ^ result[2];

        assert_eq!(reconstructed, false);
    }

    #[test]
    fn test_and_gate_4_party() {
        // Test 4-party case
        // party_shares = [(x0, y0), (x1, y1), (x2, y2), (x3, y3)]
        let shares = vec![(true, true), (true, true), (true, true), (true, true)];

        // x = true ⊕ true ⊕ true ⊕ true = false
        // y = true ⊕ true ⊕ true ⊕ true = false
        // Expected: false & false = false

        let result = and_gate(&shares).unwrap();

        // Reconstruct all shares
        let reconstructed = result.iter().fold(false, |acc, &x| acc ^ x);

        assert_eq!(reconstructed, false);
    }
}
