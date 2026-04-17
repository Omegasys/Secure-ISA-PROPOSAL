fn sign(privkey: u8, message: &[u8]) -> Vec<u8> {
    message.iter().map(|b| b ^ privkey).collect()
}

fn verify(pubkey: u8, message: &[u8], signature: &[u8]) -> bool {
    let recovered: Vec<u8> = signature.iter().map(|b| b ^ pubkey).collect();
    recovered == message
}

#[test]
fn test_pgp_verify_valid() {
    let key = 0xAA;
    let msg = b"secure message";

    let sig = sign(key, msg);

    assert!(verify(key, msg, &sig));
}

#[test]
fn test_pgp_verify_invalid_signature() {
    let key = 0xAA;
    let msg = b"secure message";

    let mut sig = sign(key, msg);
    sig[0] ^= 0xFF; // tamper

    assert!(!verify(key, msg, &sig));
}
