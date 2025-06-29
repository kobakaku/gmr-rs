pub fn not_gate(party_id: u8, share: bool) -> bool {
    if party_id == 0 {
        !share
    } else {
        share
    }
}
