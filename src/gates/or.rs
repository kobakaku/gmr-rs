use crate::gates::and_gate;
use anyhow::Result;

/// OR gate implementation using De Morgan's law and OT-based AND gate
/// x | y = ~(~x & ~y)
pub fn or_gate(alice_x: bool, alice_y: bool, bob_x: bool, bob_y: bool) -> Result<(bool, bool)> {
    // Step 1: Apply De Morgan's law - NOT x and NOT y
    // In GMW, NOT is implemented by having one party flip the bit
    let alice_not_x = !alice_x; // Alice flips her x share
    let bob_not_x = bob_x; // Bob keeps his x share unchanged

    let alice_not_y = !alice_y; // Alice flips her y share
    let bob_not_y = bob_y; // Bob keeps his y share unchanged

    // Step 2: Compute (~x & ~y) using OT-based AND gate
    let (alice_and_result, bob_and_result) =
        and_gate(alice_not_x, alice_not_y, bob_not_x, bob_not_y)?;

    // Step 3: Apply final NOT operation - ~(~x & ~y)
    // Again, Alice flips and Bob keeps unchanged
    let alice_final = !alice_and_result;
    let bob_final = bob_and_result;

    Ok((alice_final, bob_final))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{reconstruct_shares, secret_share};

    #[test]
    fn test_or_gate_all_combinations() -> Result<()> {
        // Test all input combinations
        let test_cases = [
            (false, false, false),
            (false, true, true),
            (true, false, true),
            (true, true, true),
        ];

        for (input1, input2, expected) in test_cases {
            // Create secret shares
            let (alice_x, bob_x) = secret_share(input1);
            let (alice_y, bob_y) = secret_share(input2);

            // Execute OR gate with OT
            let (alice_result, bob_result) = or_gate(alice_x, alice_y, bob_x, bob_y)?;

            // Reconstruct result
            let actual = reconstruct_shares(alice_result, bob_result);

            assert_eq!(
                actual, expected,
                "OR({}, {}) = {} but got {}",
                input1, input2, expected, actual
            );
        }

        Ok(())
    }

    #[test]
    fn test_de_morgan_law() {
        // De Morgan's law: x | y = ~(~x & ~y)
        let test_cases = [(false, false), (false, true), (true, false), (true, true)];

        for (x, y) in test_cases {
            let or_result = x | y;
            let de_morgan_result = !(!x & !y);
            assert_eq!(
                or_result, de_morgan_result,
                "De Morgan's law failed for x={}, y={}",
                x, y
            );
        }
    }
}
