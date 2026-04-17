fn encrypt(key: u8, data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

#[test]
fn test_key_reuse_leakage() {
    let key = 0xAA;

    let p1 = b"AAAA";
    let p2 = b"BBBB";

    let c1 = encrypt(key, p1);
    let c2 = encrypt(key, p2);

    // XOR of ciphertexts reveals XOR of plaintexts
    let leaked: Vec<u8> = c1.iter().zip(c2.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    let expected: Vec<u8> = p1.iter().zip(p2.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    assert_eq!(leaked, expected);
}
