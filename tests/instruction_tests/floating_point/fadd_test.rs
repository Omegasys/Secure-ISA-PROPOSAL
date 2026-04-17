fn fadd(a: f64, b: f64) -> f64 {
    a + b
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-10
}

#[test]
fn test_fadd_basic() {
    assert!(approx_eq(fadd(1.5, 2.5), 4.0));
}

#[test]
fn test_fadd_zero() {
    assert!(approx_eq(fadd(5.0, 0.0), 5.0));
}

#[test]
fn test_fadd_infinity() {
    assert!(fadd(f64::INFINITY, 1.0).is_infinite());
}
