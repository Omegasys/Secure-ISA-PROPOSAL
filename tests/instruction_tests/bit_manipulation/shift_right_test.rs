fn shift_right(value: u64, shift: u8) -> u64 {
    value.wrapping_shr(shift as u32)
}

#[test]
fn test_shift_right_basic() {
    assert_eq!(shift_right(4, 1), 2);
}

#[test]
fn test_shift_right_zero() {
    assert_eq!(shift_right(5, 0), 5);
}

#[test]
fn test_shift_right_large() {
    assert_eq!(shift_right(1, 63), 0);
}
