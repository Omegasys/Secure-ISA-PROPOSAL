fn fmul(a: f64, b: f64) -> f64 {
    a * b
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-10
}

#[test]
fn test_fmul_basic() {
    assert!(approx_eq(fmul(2.0, 3.5), 7.0));
}

#[test]
fn test_fmul_zero() {
    assert!(approx_eq(fmul(5.0, 0.0), 0.0));
}

#[test]
fn test_fmul_infinity() {
    assert!(fmul(f64::INFINITY, 2.0).is_infinite());
}
