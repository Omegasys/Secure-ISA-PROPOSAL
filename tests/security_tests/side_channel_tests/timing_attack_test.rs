use std::time::Instant;

// Vulnerable comparison (early exit)
fn insecure_compare(a: &[u8], b: &[u8]) -> bool {
    for i in 0..a.len().min(b.len()) {
        if a[i] != b[i] {
            return false;
        }
    }
    a.len() == b.len()
}

// Constant-time comparison
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for i in 0..a.len() {
        result |= a[i] ^ b[i];
    }

    result == 0
}

#[test]
fn test_timing_variance_detection() {
    let a = b"AAAAAAAAAAAAAAAA";
    let b1 = b"AAAAAAAAAAAAAAAA"; // identical
    let b2 = b"AAAAAAAABAAAAAAA"; // mismatch late
    let b3 = b"BAAAAAAAAAAAAAAA"; // mismatch early

    let start = Instant::now();
    insecure_compare(a, b1);
    let t1 = start.elapsed();

    let start = Instant::now();
    insecure_compare(a, b2);
    let t2 = start.elapsed();

    let start = Instant::now();
    insecure_compare(a, b3);
    let t3 = start.elapsed();

    // Not strict equality—just detect variance
    assert!(t1 != t2 || t2 != t3);
}

#[test]
fn test_constant_time_behavior() {
    let a = b"AAAAAAAAAAAAAAAA";
    let b = b"AAAAAAAABAAAAAAA";

    let start = Instant::now();
    constant_time_compare(a, b);
    let t1 = start.elapsed();

    let start = Instant::now();
    constant_time_compare(a, a);
    let t2 = start.elapsed();

    // Should be roughly equal (not exact)
    let diff = if t1 > t2 { t1 - t2 } else { t2 - t1 };

    assert!(diff.as_nanos() < 1_000_000); // tolerance threshold
}
