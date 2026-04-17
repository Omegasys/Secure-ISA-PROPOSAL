fn fsqrt(a: f64) -> f64 {
    a.sqrt()
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-10
}

#[test]
fn test_fsqrt_basic() {
    assert!(approx_eq(fsqrt(9.0), 3.0));
}

#[test]
fn test_fsqrt_zero() {
    assert!(approx_eq(fsqrt(0.0), 0.0));
}

#[test]
fn test_fsqrt_negative() {
    let result = fsqrt(-1.0);
    assert!(result.is_nan());
}
