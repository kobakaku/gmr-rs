use anyhow::Result;
use oblivious_transfer_rs::{
    Choice, OTReceiver, OTSender, ReceiverEncryptedValues, SenderMaskedMessages, SenderPublicKey,
};

/// OT wrapper for GMW protocol
/// Makes it easier to handle bit-based OT
pub struct BitOT;

impl BitOT {
    /// Execute 1-out-of-2 OT (single bit version)
    ///
    /// # Arguments
    /// * `sender_bits` - (m0, m1) sender's two bit messages
    /// * `receiver_choice` - receiver's choice bit (0 or 1)
    ///
    /// # Returns
    /// * Selected bit value
    pub fn execute(
        sender_bits: (bool, bool),
        receiver_choice: bool,
    ) -> Result<(OTSenderState, OTReceiverState)> {
        // Convert bits to Vec<u8>
        let m0 = vec![sender_bits.0 as u8];
        let m1 = vec![sender_bits.1 as u8];

        // Create OT sender
        let mut sender = OTSender::new(m0, m1)?;

        // Create OT receiver
        let choice = if receiver_choice {
            Choice::One
        } else {
            Choice::Zero
        };
        let mut receiver = OTReceiver::new(choice);

        // Phase 1: Sender generates keys
        let sender_pk = sender.generate_keys()?;

        // Phase 2: Receiver generates encrypted values
        let encrypted_values = receiver.generate_encrypted_values(sender_pk.clone())?;

        // Phase 3: Sender creates masked messages
        let masked_messages = sender.create_masked_messages(encrypted_values.clone())?;

        // Phase 4: Extract the message for the receiver
        let result = receiver.extract_message(masked_messages.clone())?;
        let received_bit = result.first().copied().unwrap_or(0) != 0;

        // Return states including the received bit
        Ok((
            OTSenderState {
                sender_pk,
                masked_messages,
            },
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
}

/// OT sender state
pub struct OTSenderState {
    pub sender_pk: SenderPublicKey,
    pub masked_messages: SenderMaskedMessages,
}

/// OT receiver state
pub struct OTReceiverState {
    pub receiver: OTReceiver,
    pub encrypted_values: ReceiverEncryptedValues,
    pub received_bit: bool,
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
}
