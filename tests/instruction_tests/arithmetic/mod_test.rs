fn modulo(a: u64, b: u64) -> Option<u64> {
    if b == 0 {
        None
    } else {
        Some(a % b)
    }
}

#[test]
fn test_mod_basic() {
    assert_eq!(modulo(10, 3), Some(1));
}

#[test]
fn test_mod_zero_divisor() {
    assert_eq!(modulo(10, 0), None);
}

#[test]
fn test_mod_exact_division() {
    assert_eq!(modulo(9, 3), Some(0));
}
