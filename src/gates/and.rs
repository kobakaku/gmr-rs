use crate::ot::BitOT;
use anyhow::Result;
use rand::random;

/// Secure AND gate implementation using 1-out-of-4 OT
///
/// GMW protocol for AND: z = x & y = (x0 ⊕ x1) & (y0 ⊕ y1)
/// = x0&y0 ⊕ x0&y1 ⊕ x1&y0 ⊕ x1&y1
///
/// To avoid leaking Alice's x0 to Bob, we use:
/// - Alice generates random r (her output share)
/// - Bob receives his share via 1-out-of-4 OT based on his (x1, y1)
/// - The 4 messages are precomputed for Bob's possible inputs
pub fn and_gate(alice_x: bool, alice_y: bool, bob_x: bool, bob_y: bool) -> Result<(bool, bool)> {
    // Alice generates random bit r (this becomes her output share)
    let alice_share = random::<bool>();

    // Alice's local computation: x0 & y0
    let alice_local = alice_x & alice_y;

    // Precompute messages for Bob's possible inputs (x1, y1)
    // Bob should receive: z ⊕ r where z is the full AND result
    let messages = (
        // (x1=0, y1=0): z = x0&y0, so Bob gets x0&y0 ⊕ r
        alice_local ^ alice_share,
        // (x1=0, y1=1): z = x0&y0 ⊕ x0&y1 = x0&y0 ⊕ x0 = x0&(y0⊕1), so Bob gets x0&(y0⊕1) ⊕ r
        alice_local ^ alice_x ^ alice_share,
        // (x1=1, y1=0): z = x0&y0 ⊕ x1&y0 = x0&y0 ⊕ y0 = y0&(x0⊕1), so Bob gets y0&(x0⊕1) ⊕ r
        alice_local ^ alice_y ^ alice_share,
        // (x1=1, y1=1): z = x0&y0 ⊕ x0&y1 ⊕ x1&y0 ⊕ x1&y1 = x0&y0 ⊕ x0 ⊕ y0 ⊕ 1, so Bob gets that ⊕ r
        alice_local ^ alice_x ^ alice_y ^ true ^ alice_share,
    );

    // Bob's choice bits are his input shares
    let bob_choice = (bob_x, bob_y);

    // Execute 1-out-of-4 OT
    let bob_share = BitOT::execute_1_out_of_4(messages, bob_choice)?;

    Ok((alice_share, bob_share))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{reconstruct_shares, secret_share};

    #[test]
    fn test_and_gate_with_ot_all_combinations() -> Result<()> {
        // Test all input combinations
        let test_cases = [
            (false, false, false),
            (false, true, false),
            (true, false, false),
            (true, true, true),
        ];

        for (input1, input2, expected) in test_cases {
            // Create secret shares
            let (alice_x, bob_x) = secret_share(input1);
            let (alice_y, bob_y) = secret_share(input2);

            // Execute AND gate with OT
            let (alice_result, bob_result) = and_gate(alice_x, alice_y, bob_x, bob_y)?;

            // Reconstruct result
            let actual = reconstruct_shares(alice_result, bob_result);

            assert_eq!(
                actual, expected,
                "AND({}, {}) = {} but got {}",
                input1, input2, expected, actual
            );
        }

        Ok(())
    }
}
