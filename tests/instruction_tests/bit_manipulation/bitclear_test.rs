fn bitclear(value: u64, bit: u8) -> u64 {
    value & !(1u64 << bit)
}

#[test]
fn test_bitclear_basic() {
    assert_eq!(bitclear(0b1111, 1), 0b1101);
}

#[test]
fn test_bitclear_already_zero() {
    assert_eq!(bitclear(0b1101, 1), 0b1101);
}

#[test]
fn test_bitclear_high_bit() {
    let val = 1u64 << 63;
    assert_eq!(bitclear(val, 63), 0);
}
