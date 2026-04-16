pub fn encrypt(pubkey: u64, data: u64) -> u64 {
    data.wrapping_mul(pubkey)
}

pub fn decrypt(privkey: u64, data: u64) -> u64 {
    data.wrapping_div(privkey)
}
