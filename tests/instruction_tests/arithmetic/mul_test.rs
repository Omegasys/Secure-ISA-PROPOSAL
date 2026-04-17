use crate::execution::alu::mul;

#[test]
fn test_mul_basic() {
    assert_eq!(mul(4, 3), 12);
}

#[test]
fn test_mul_zero() {
    assert_eq!(mul(5, 0), 0);
}

#[test]
fn test_mul_overflow() {
    let result = mul(u64::MAX, 2);
    assert!(result < u64::MAX); // wrapped result
}
