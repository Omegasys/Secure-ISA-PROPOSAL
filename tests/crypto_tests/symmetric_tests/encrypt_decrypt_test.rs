fn encrypt(key: u8, data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

fn decrypt(key: u8, data: &[u8]) -> Vec<u8> {
    encrypt(key, data) // XOR is symmetric
}

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let key = 0xAA;
    let plaintext = b"secret data";

    let ciphertext = encrypt(key, plaintext);
    let recovered = decrypt(key, &ciphertext);

    assert_eq!(recovered, plaintext);
}
