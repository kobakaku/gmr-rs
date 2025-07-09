use anyhow::Result;
use oblivious_transfer_rs::{
    Choice, OTReceiver, OTSender, ReceiverEncryptedValues, SenderMaskedMessages, SenderPublicKey,
};

/// OT wrapper for GMW protocol
/// Makes it easier to handle bit-based OT
pub struct BitOT;

impl BitOT {
    /// Execute 1-out-of-2 OT for single bit messages
    ///
    /// # Arguments
    /// * `messages` - (m0, m1) two bit messages
    /// * `choice` - selection bit (0 for m0, 1 for m1)
    ///
    /// # Returns
    /// * Sender state and receiver state containing selected bit
    pub fn execute(
        messages: (bool, bool),
        choice: bool,
    ) -> Result<(OTSenderState, OTReceiverState)> {
        // Convert bits to byte vectors
        let m0 = vec![messages.0 as u8];
        let m1 = vec![messages.1 as u8];

        // Execute core OT protocol
        let (sender_state, receiver, encrypted_values, result) =
            Self::execute_ot_core(m0, m1, choice)?;
        let received_bit = result.first().copied().unwrap_or(0) != 0;

        Ok((
            sender_state,
            OTReceiverState {
                receiver,
                encrypted_values,
                received_bit,
            },
        ))
    }

    /// Extract message on receiver side
    pub fn extract_bit(
        receiver_state: &OTReceiverState,
        masked_messages: SenderMaskedMessages,
    ) -> Result<bool> {
        let result = receiver_state.receiver.extract_message(masked_messages)?;

        // Convert Vec<u8> to bit
        Ok(result.first().copied().unwrap_or(0) != 0)
    }

    /// Execute 1-out-of-4 OT using two sequential 1-out-of-2 OTs
    ///
    /// # Arguments
    /// * `messages` - (m00, m01, m10, m11) four bit messages indexed by 2-bit choice
    /// * `choice_bits` - (b0, b1) two choice bits where b0||b1 selects message index
    ///
    /// # Returns
    /// * Selected bit value m_{b0,b1}
    pub fn execute_1_out_of_4(
        messages: (bool, bool, bool, bool),
        choice_bits: (bool, bool),
    ) -> Result<bool> {
        let (m00, m01, m10, m11) = messages;
        let (b0, b1) = choice_bits;

        // First OT: Choose between message pairs based on b0
        let message_pairs = ((m00, m01), (m10, m11));
        let (_sender_state1, receiver_state1) = Self::execute_bit_pairs(message_pairs, b0)?;

        // Second OT: Choose from selected pair based on b1
        let selected_pair = receiver_state1.received_pair;
        let (_sender_state2, receiver_state2) = Self::execute(selected_pair, b1)?;

        Ok(receiver_state2.received_bit)
    }

    /// Execute 1-out-of-2 OT where messages are bit pairs
    ///
    /// # Arguments
    /// * `message_pairs` - ((m00, m01), (m10, m11)) two pairs of bits
    /// * `choice` - selection bit (0 for first pair, 1 for second pair)
    ///
    /// # Returns
    /// * Sender state and receiver state containing selected bit pair
    fn execute_bit_pairs(
        message_pairs: ((bool, bool), (bool, bool)),
        choice: bool,
    ) -> Result<(OTSenderState, OTReceiverPairState)> {
        let ((m00, m01), (m10, m11)) = message_pairs;

        // Convert bit pairs to byte vectors
        let pair0_bytes = vec![m00 as u8, m01 as u8];
        let pair1_bytes = vec![m10 as u8, m11 as u8];

        // Execute core OT protocol
        let (sender_state, receiver, encrypted_values, result) =
            Self::execute_ot_core(pair0_bytes, pair1_bytes, choice)?;

        // Convert result bytes back to bit pair
        let received_pair = (
            result.first().copied().unwrap_or(0) != 0,
            result.get(1).copied().unwrap_or(0) != 0,
        );

        Ok((
            sender_state,
            OTReceiverPairState {
                receiver,
                encrypted_values,
                received_pair,
            },
        ))
    }

    /// Execute the core OT protocol with byte messages
    ///
    /// # Arguments
    /// * `m0` - first message as bytes
    /// * `m1` - second message as bytes
    /// * `choice` - receiver's choice bit
    ///
    /// # Returns
    /// * Sender state, receiver instance, encrypted values, and selected message
    fn execute_ot_core(
        m0: Vec<u8>,
        m1: Vec<u8>,
        choice: bool,
    ) -> Result<(OTSenderState, OTReceiver, ReceiverEncryptedValues, Vec<u8>)> {
        // Initialize OT participants
        let mut sender = OTSender::new(m0, m1)?;
        let choice_enum = if choice { Choice::One } else { Choice::Zero };
        let mut receiver = OTReceiver::new(choice_enum);

        // Execute OT protocol phases
        let sender_pk = sender.generate_keys()?;
        let encrypted_values = receiver.generate_encrypted_values(sender_pk.clone())?;
        let masked_messages = sender.create_masked_messages(encrypted_values.clone())?;
        let result = receiver.extract_message(masked_messages.clone())?;

        Ok((
            OTSenderState {
                sender_pk,
                masked_messages,
            },
            receiver,
            encrypted_values,
            result,
        ))
    }
}

/// OT sender state
pub struct OTSenderState {
    pub sender_pk: SenderPublicKey,
    pub masked_messages: SenderMaskedMessages,
}

/// OT receiver state for single bit messages
pub struct OTReceiverState {
    pub receiver: OTReceiver,
    pub encrypted_values: ReceiverEncryptedValues,
    pub received_bit: bool,
}

/// OT receiver state for bit pair messages
pub struct OTReceiverPairState {
    pub receiver: OTReceiver,
    pub encrypted_values: ReceiverEncryptedValues,
    pub received_pair: (bool, bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_ot() -> Result<()> {
        // Basic OT test
        let sender_bits = (false, true);
        let receiver_choice = true;

        let (sender_state, receiver_state) = BitOT::execute(sender_bits, receiver_choice)?;
        let result = BitOT::extract_bit(&receiver_state, sender_state.masked_messages)?;

        assert_eq!(result, true); // choice=1 so we get m1=true
        Ok(())
    }

    #[test]
    fn test_1_out_of_4_ot() -> Result<()> {
        // Test 1-out-of-4 OT
        let messages = (false, true, true, false); // m00, m01, m10, m11

        // Test all possible choices
        let test_cases = [
            ((false, false), false), // choice 00 -> m00
            ((false, true), true),   // choice 01 -> m01
            ((true, false), true),   // choice 10 -> m10
            ((true, true), false),   // choice 11 -> m11
        ];

        for (choice_bits, expected) in test_cases {
            let result = BitOT::execute_1_out_of_4(messages, choice_bits)?;
            assert_eq!(result, expected, "Failed for choice {:?}", choice_bits);
        }

        Ok(())
    }
}
