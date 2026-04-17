fn is_valid_key(key: u64) -> bool {
    key != 0
}

#[test]
fn test_valid_key() {
    assert!(is_valid_key(12345));
}

#[test]
fn test_invalid_key_zero() {
    assert!(!is_valid_key(0));
}
