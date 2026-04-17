fn strlen(s: &[u8]) -> usize {
    s.len()
}

#[test]
fn test_strlen_basic() {
    assert_eq!(strlen(b"hello"), 5);
}

#[test]
fn test_strlen_empty() {
    assert_eq!(strlen(b""), 0);
}
