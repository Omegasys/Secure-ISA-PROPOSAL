fn xor(a: u64, b: u64) -> u64 {
    a ^ b
}

#[test]
fn test_xor_basic() {
    assert_eq!(xor(0b1100, 0b1010), 0b0110);
}

#[test]
fn test_xor_self() {
    assert_eq!(xor(0b1111, 0b1111), 0);
}

#[test]
fn test_xor_zero() {
    assert_eq!(xor(0b1010, 0), 0b1010);
}
