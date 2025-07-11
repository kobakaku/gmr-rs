use crate::ot::BitOT;
use anyhow::Result;
use rand::random;

/// 3-party AND gate implementation for local simulation (Phase 1)
///
/// In 3-party GMW: z = x AND y where:
/// - x = x0 ⊕ x1 ⊕ x2
/// - y = y0 ⊕ y1 ⊕ y2
///
/// The AND computation expands to:
/// z = (x0·y0 ⊕ x1·y1 ⊕ x2·y2) ⊕ ((x0·y1 ⊕ x1·y0) ⊕ (x0·y2 ⊕ x2·y0) ⊕ (x1·y2 ⊕ x2·y1))
///
/// Each party Pi computes:
/// - Local term: xi·yi (computed locally)
/// - Cross terms: xi·yj ⊕ xj·yi with other parties (using OT)
pub fn and_gate_3party_local(
    party_shares: [(bool, bool); 3], // Each party's (x_share, y_share)
) -> Result<[bool; 3]> {
    let [(x0, y0), (x1, y1), (x2, y2)] = party_shares;

    // Each party computes their local term xi·yi
    let local_terms = [x0 & y0, x1 & y1, x2 & y2];

    // Simulate OT between each pair of parties
    // We need 3 OT sessions: (0,1), (0,2), (1,2)

    // OT between Party 0 and Party 1
    let (cross_01_for_p0, cross_01_for_p1) = compute_cross_term_ot((x0, y0), (x1, y1), 0, 1)?;

    // OT between Party 0 and Party 2
    let (cross_02_for_p0, cross_02_for_p2) = compute_cross_term_ot((x0, y0), (x2, y2), 0, 2)?;

    // OT between Party 1 and Party 2
    let (cross_12_for_p1, cross_12_for_p2) = compute_cross_term_ot((x1, y1), (x2, y2), 1, 2)?;

    // Each party combines their local term with cross terms
    let result_shares = [
        local_terms[0] ^ cross_01_for_p0 ^ cross_02_for_p0, // Party 0
        local_terms[1] ^ cross_01_for_p1 ^ cross_12_for_p1, // Party 1
        local_terms[2] ^ cross_02_for_p2 ^ cross_12_for_p2, // Party 2
    ];

    Ok(result_shares)
}

/// Compute cross term xi·yj ⊕ xj·yi between two parties using OT
/// Returns shares for both parties such that:
/// share_i ⊕ share_j = xi·yj ⊕ xj·yi
fn compute_cross_term_ot(
    party_i: (bool, bool), // (xi, yi)
    party_j: (bool, bool), // (xj, yj)
    i: usize,
    j: usize,
) -> Result<(bool, bool)> {
    let (xi, yi) = party_i;
    let (xj, yj) = party_j;

    // Party i acts as sender, party j as receiver
    // We need to compute xi·yj ⊕ xj·yi securely

    // Party i generates random bit ri (will be party i's share)
    let ri = random::<bool>();

    // Party j needs to receive: (xi·yj ⊕ xj·yi) ⊕ ri
    // Using 1-out-of-4 OT based on (xj, yj) as choice bits

    // Party i prepares 4 messages for all possible (xj, yj) values:
    // (0,0): xi·0 ⊕ 0·yi ⊕ ri = 0 ⊕ ri = ri
    // (0,1): xi·1 ⊕ 0·yi ⊕ ri = xi ⊕ ri
    // (1,0): xi·0 ⊕ 1·yi ⊕ ri = yi ⊕ ri
    // (1,1): xi·1 ⊕ 1·yi ⊕ ri = xi ⊕ yi ⊕ ri

    let messages = (
        ri,           // (0,0)
        xi ^ ri,      // (0,1)
        yi ^ ri,      // (1,0)
        xi ^ yi ^ ri, // (1,1)
    );

    // Execute 1-out-of-4 OT (simulated locally for Phase 1)
    let rj = BitOT::execute_1_out_of_4(messages, (xj, yj))?;

    // Return shares based on party order
    if i < j {
        Ok((ri, rj)) // Party i gets ri, party j gets rj
    } else {
        Ok((rj, ri)) // Swap if parties are in opposite order
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::party::{reconstruct_shares_3party, secret_share_3party};

    #[test]
    fn test_3party_and_gate_local() -> Result<()> {
        // Test all input combinations
        for x in [false, true] {
            for y in [false, true] {
                // Create 3-party shares
                let (x0, x1, x2) = secret_share_3party(x);
                let (y0, y1, y2) = secret_share_3party(y);

                // Execute 3-party AND gate
                let party_shares = [(x0, y0), (x1, y1), (x2, y2)];
                let result_shares = and_gate_3party_local(party_shares)?;

                // Reconstruct result
                let result =
                    reconstruct_shares_3party(result_shares[0], result_shares[1], result_shares[2]);

                assert_eq!(
                    result,
                    x & y,
                    "3-party AND({}, {}) failed: expected {}, got {}",
                    x,
                    y,
                    x & y,
                    result
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_cross_term_correctness() -> Result<()> {
        // Test that cross terms compute correctly
        for xi in [false, true] {
            for yi in [false, true] {
                for xj in [false, true] {
                    for yj in [false, true] {
                        let (share_i, share_j) = compute_cross_term_ot((xi, yi), (xj, yj), 0, 1)?;

                        // Verify that shares reconstruct to the correct cross term
                        let reconstructed = share_i ^ share_j;
                        let expected = (xi & yj) ^ (xj & yi);

                        assert_eq!(
                            reconstructed, expected,
                            "Cross term failed for ({},{}) and ({},{})",
                            xi, yi, xj, yj
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
