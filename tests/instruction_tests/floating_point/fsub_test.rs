fn fsub(a: f64, b: f64) -> f64 {
    a - b
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-10
}

#[test]
fn test_fsub_basic() {
    assert!(approx_eq(fsub(5.5, 2.5), 3.0));
}

#[test]
fn test_fsub_negative() {
    assert!(approx_eq(fsub(2.0, 5.0), -3.0));
}

#[test]
fn test_fsub_nan() {
    let result = fsub(f64::NAN, 1.0);
    assert!(result.is_nan());
}
