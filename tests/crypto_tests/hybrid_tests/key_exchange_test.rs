fn derive_shared(a: u8, b: u8) -> u8 {
    a ^ b
}

#[test]
fn test_key_exchange_shared_secret() {
    let alice_priv = 0x12;
    let bob_priv = 0x34;

    let shared1 = derive_shared(alice_priv, bob_priv);
    let shared2 = derive_shared(bob_priv, alice_priv);

    assert_eq!(shared1, shared2);
}

#[test]
fn test_key_exchange_symmetry() {
    let a = 0x55;
    let b = 0xAA;

    assert_eq!(derive_shared(a, b), derive_shared(b, a));
}
