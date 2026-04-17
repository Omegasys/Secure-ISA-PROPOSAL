use crate::execution::alu::div;

#[test]
fn test_div_basic() {
    assert_eq!(div(10, 2), Some(5));
}

#[test]
fn test_div_by_one() {
    assert_eq!(div(7, 1), Some(7));
}

#[test]
fn test_div_by_zero() {
    assert_eq!(div(10, 0), None);
}
