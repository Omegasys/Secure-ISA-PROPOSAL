fn bitreverse(mut value: u64) -> u64 {
    let mut result = 0;

    for _ in 0..64 {
        result <<= 1;
        result |= value & 1;
        value >>= 1;
    }

    result
}

#[test]
fn test_bitreverse_basic() {
    let input = 0b0001u64;
    let output = bitreverse(input);

    assert_eq!(output >> 63, 1);
}

#[test]
fn test_bitreverse_symmetry() {
    let value = 0xF0F0F0F0F0F0F0F0;
    assert_eq!(bitreverse(bitreverse(value)), value);
}

#[test]
fn test_bitreverse_zero() {
    assert_eq!(bitreverse(0), 0);
}
