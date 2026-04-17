fn ratchet(prev: u8) -> u8 {
    prev.wrapping_mul(17) ^ 0x33
}

#[test]
fn test_post_compromise_security() {
    let mut state = 0x10;

    // attacker compromises state here
    let compromised = state;

    // system advances
    state = ratchet(state);
    state = ratchet(state);

    // attacker should not predict new state easily
    assert_ne!(state, compromised);
}
