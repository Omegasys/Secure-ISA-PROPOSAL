fn sign(privkey: u8, message: &[u8]) -> Vec<u8> {
    message.iter().map(|b| b ^ privkey).collect()
}

#[test]
fn test_pgp_sign_basic() {
    let privkey = 0xAA;
    let msg = b"hello world";

    let signature = sign(privkey, msg);

    assert_eq!(signature.len(), msg.len());
}

#[test]
fn test_pgp_sign_deterministic() {
    let privkey = 0xAA;
    let msg = b"data";

    let sig1 = sign(privkey, msg);
    let sig2 = sign(privkey, msg);

    assert_eq!(sig1, sig2); // deterministic signing
}
