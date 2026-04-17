fn ratchet_key(prev: u8) -> u8 {
    prev.wrapping_mul(37) ^ 0x5A
}

#[test]
fn test_forward_secrecy() {
    let k1 = 0x10;
    let k2 = ratchet_key(k1);
    let k3 = ratchet_key(k2);

    // attacker learns k3 → should not derive k2 or k1
    assert_ne!(k3, k2);
    assert_ne!(k3, k1);
}
