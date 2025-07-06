use crate::ot::BitOT;
use anyhow::Result;

/// AND gate implementation using oblivious transfer
///
/// GMW protocol for AND: x & y = (x0 ⊕ x1) & (y0 ⊕ y1)
/// = x0&y0 ⊕ x0&y1 ⊕ x1&y0 ⊕ x1&y1
///
/// Party 0 computes x0&y0 locally
/// Party 1 computes x1&y1 locally  
/// Cross terms x0&y1 and x1&y0 computed via OT
pub fn and_gate(alice_x: bool, alice_y: bool, bob_x: bool, bob_y: bool) -> Result<(bool, bool)> {
    // First OT: Alice (party 0) sends her x share to Bob based on his y share
    // Bob chooses between Alice's (0, x0) based on his y share
    let alice_messages = (false, alice_x); // Messages: [0, x0]
    let bob_choice = bob_y; // Bob's choice bit is his y share

    let (_ot1_sender_state, ot1_receiver_state) = BitOT::execute(alice_messages, bob_choice)?;

    // Bob receives: x0 if y1=1, 0 if y1=0 → x0&y1
    let cross_term1 = ot1_receiver_state.received_bit;

    // Second OT: Bob (party 1) sends his x share to Alice based on her y share
    // Alice chooses between Bob's (0, x1) based on her y share
    let bob_messages = (false, bob_x); // Messages: [0, x1]
    let alice_choice = alice_y; // Alice's choice bit is her y share

    let (_ot2_sender_state, ot2_receiver_state) = BitOT::execute(bob_messages, alice_choice)?;

    // Alice receives: x1 if y0=1, 0 if y0=0 → x1&y0
    let cross_term2 = ot2_receiver_state.received_bit;

    // Compute local terms
    let alice_local = alice_x & alice_y; // x0 & y0
    let bob_local = bob_x & bob_y; // x1 & y1

    // Combine all terms: x&y = (x0&y0) ⊕ (x0&y1) ⊕ (x1&y0) ⊕ (x1&y1)
    // Alice gets: (x0&y0) ⊕ (x1&y0) = alice_local ⊕ cross_term2
    // Bob gets: (x0&y1) ⊕ (x1&y1) = cross_term1 ⊕ bob_local
    let alice_result = alice_local ^ cross_term2;
    let bob_result = cross_term1 ^ bob_local;

    Ok((alice_result, bob_result))
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
