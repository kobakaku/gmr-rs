/// 3-party NOT gate
/// Party 0 flips their share, others keep theirs unchanged
pub fn not_gate_3party(party_id: u32, share: bool) -> bool {
    if party_id == 0 {
        !share
    } else {
        share
    }
}
