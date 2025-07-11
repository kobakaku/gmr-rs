/// 3-party XOR gate - each party computes locally
/// Since XOR is linear in GF(2), no communication is needed
pub fn xor_gate_3party(x_share: bool, y_share: bool) -> bool {
    x_share ^ y_share
}
