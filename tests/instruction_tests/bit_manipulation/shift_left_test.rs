fn shift_left(value: u64, shift: u8) -> u64 {
    value.wrapping_shl(shift as u32)
}

#[test]
fn test_shift_left_basic() {
    assert_eq!(shift_left(1, 1), 2);
}

#[test]
fn test_shift_left_zero() {
    assert_eq!(shift_left(5, 0), 5);
}

#[test]
fn test_shift_left_overflow() {
    let result = shift_left(1, 64);
    assert_eq!(result, 1); // wrapping behavior (platform dependent but expected here)
}
