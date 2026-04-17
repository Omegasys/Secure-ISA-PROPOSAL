fn and(a: u64, b: u64) -> u64 {
    a & b
}

#[test]
fn test_and_basic() {
    assert_eq!(and(0b1100, 0b1010), 0b1000);
}

#[test]
fn test_and_zero() {
    assert_eq!(and(0b1111, 0), 0);
}

#[test]
fn test_and_all_ones() {
    assert_eq!(and(u64::MAX, u64::MAX), u64::MAX);
}
