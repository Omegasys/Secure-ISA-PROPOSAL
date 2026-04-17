use crate::execution::alu::add;
use std::time::Instant;

#[test]
fn test_constant_time_add_behavior() {
    // NOTE:
    // This is a *heuristic* test. True constant-time guarantees
    // cannot be proven with timing alone, but large deviations
    // can indicate problems.

    let iterations = 1_000_000;

    let start_a = Instant::now();
    for _ in 0..iterations {
        add(1, 1);
    }
    let duration_a = start_a.elapsed();

    let start_b = Instant::now();
    for _ in 0..iterations {
        add(u64::MAX, 1);
    }
    let duration_b = start_b.elapsed();

    let diff = if duration_a > duration_b {
        duration_a - duration_b
    } else {
        duration_b - duration_a
    };

    // Allow small timing noise threshold
    assert!(diff.as_nanos() < 5_000_000, "Potential timing variance detected");
}
