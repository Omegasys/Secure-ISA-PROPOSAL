use crate::execution::alu::add;

#[test]
fn test_add_basic() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_add_zero() {
    assert_eq!(add(0, 5), 5);
}

#[test]
fn test_add_overflow() {
    let result = add(u64::MAX, 1);
    assert_eq!(result, 0); // wrapping behavior
}
