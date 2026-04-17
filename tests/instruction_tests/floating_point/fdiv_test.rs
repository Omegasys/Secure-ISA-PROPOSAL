fn fdiv(a: f64, b: f64) -> f64 {
    a / b
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-10
}

#[test]
fn test_fdiv_basic() {
    assert!(approx_eq(fdiv(10.0, 2.0), 5.0));
}

#[test]
fn test_fdiv_fraction() {
    assert!(approx_eq(fdiv(1.0, 2.0), 0.5));
}

#[test]
fn test_fdiv_by_zero() {
    let result = fdiv(1.0, 0.0);
    assert!(result.is_infinite());
}

#[test]
fn test_fdiv_nan() {
    let result = fdiv(0.0, 0.0);
    assert!(result.is_nan());
}
