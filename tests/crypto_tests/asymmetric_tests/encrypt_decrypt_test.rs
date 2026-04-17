fn encrypt(pubkey: u64, msg: u64) -> u64 {
    msg ^ pubkey
}

fn decrypt(privkey: u64, cipher: u64) -> u64 {
    cipher ^ privkey
}

#[test]
fn test_asymmetric_encrypt_decrypt() {
    let key = 0xDEADBEEF;

    let msg = 42;
    let cipher = encrypt(key, msg);
    let recovered = decrypt(key, cipher);

    assert_eq!(recovered, msg);
}
