fn bitset(value: u64, bit: u8) -> u64 {
    value | (1u64 << bit)
}

#[test]
fn test_bitset_basic() {
    assert_eq!(bitset(0b0000, 1), 0b0010);
}

#[test]
fn test_bitset_existing_bit() {
    assert_eq!(bitset(0b0010, 1), 0b0010);
}

#[test]
fn test_bitset_high_bit() {
    assert_eq!(bitset(0, 63), 1u64 << 63);
}
