fn symmetric_encrypt(key: u8, data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

fn asymmetric_encrypt(pubkey: u8, key: u8) -> u8 {
    key ^ pubkey
}

fn asymmetric_decrypt(privkey: u8, enc_key: u8) -> u8 {
    enc_key ^ privkey
}

#[test]
fn test_hybrid_encryption_flow() {
    let pubkey = 0xAA;
    let privkey = 0xAA;

    let session_key = 0x42;
    let plaintext = b"secure message";

    let enc_key = asymmetric_encrypt(pubkey, session_key);
    let ciphertext = symmetric_encrypt(session_key, plaintext);

    let recovered_key = asymmetric_decrypt(privkey, enc_key);
    let recovered = symmetric_encrypt(recovered_key, &ciphertext);

    assert_eq!(recovered, plaintext);
}
