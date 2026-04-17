fn encrypt_block(key: u8, block: &[u8]) -> Vec<u8> {
    block.iter().map(|b| b ^ key).collect()
}

#[test]
fn test_block_tampering_detection_failure() {
    let key = 0xAA;
    let plaintext = b"block";

    let mut ciphertext = encrypt_block(key, plaintext);

    // attacker flips a bit
    ciphertext[0] ^= 0xFF;

    let decrypted: Vec<u8> = ciphertext.iter().map(|b| b ^ key).collect();

    // This WILL NOT detect tampering → demonstrates weakness
    assert_ne!(decrypted, plaintext);
}
