fn or(a: u64, b: u64) -> u64 {
    a | b
}

#[test]
fn test_or_basic() {
    assert_eq!(or(0b1100, 0b1010), 0b1110);
}

#[test]
fn test_or_zero() {
    assert_eq!(or(0b1010, 0), 0b1010);
}

#[test]
fn test_or_all_ones() {
    assert_eq!(or(u64::MAX, 0), u64::MAX);
}
