fn sign(privkey: u64, msg: u64) -> u64 {
    msg ^ privkey
}

fn verify(pubkey: u64, msg: u64, sig: u64) -> bool {
    (sig ^ pubkey) == msg
}

#[test]
fn test_signature_valid() {
    let key = 0xBEEF;

    let msg = 123;
    let sig = sign(key, msg);

    assert!(verify(key, msg, sig));
}

#[test]
fn test_signature_tampered() {
    let key = 0xBEEF;

    let msg = 123;
    let mut sig = sign(key, msg);

    sig ^= 1; // tamper

    assert!(!verify(key, msg, sig));
}
