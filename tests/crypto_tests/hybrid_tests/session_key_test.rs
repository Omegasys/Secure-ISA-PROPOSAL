fn generate_session_key(seed: u8) -> u8 {
    seed.wrapping_mul(31) ^ 0xAA
}

#[test]
fn test_session_key_uniqueness() {
    let k1 = generate_session_key(1);
    let k2 = generate_session_key(2);

    assert_ne!(k1, k2);
}

#[test]
fn test_session_key_reuse_detectable() {
    let k1 = generate_session_key(1);
    let k2 = generate_session_key(1);

    assert_eq!(k1, k2); // demonstrates reuse risk
}
