use crate::execution::alu::sub;

#[test]
fn test_sub_basic() {
    assert_eq!(sub(5, 3), 2);
}

#[test]
fn test_sub_zero() {
    assert_eq!(sub(5, 0), 5);
}

#[test]
fn test_sub_underflow() {
    let result = sub(0, 1);
    assert_eq!(result, u64::MAX); // wrapping underflow
}
