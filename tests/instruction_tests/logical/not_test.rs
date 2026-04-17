fn not(a: u64) -> u64 {
    !a
}

#[test]
fn test_not_basic() {
    assert_eq!(not(0b0000), u64::MAX);
}

#[test]
fn test_not_all_ones() {
    assert_eq!(not(u64::MAX), 0);
}

#[test]
fn test_not_pattern() {
    let value = 0b1010u64;
    let result = not(value);

    assert_eq!(result & 0b1111, 0b0101);
}
