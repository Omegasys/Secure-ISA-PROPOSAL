pub fn encrypt_block(key: &[u8; 16], data: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];

    for i in 0..16 {
        out[i] = data[i] ^ key[i]; // placeholder XOR cipher for reference model
    }

    out
}

pub fn decrypt_block(key: &[u8; 16], data: &[u8; 16]) -> [u8; 16] {
    encrypt_block(key, data)
}
