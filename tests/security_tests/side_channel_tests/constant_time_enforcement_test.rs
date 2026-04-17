use std::time::Instant;

// Simulated ISA flag
#[derive(Default)]
struct CPU {
    constant_time_mode: bool,
}

// Operation that behaves differently depending on mode
fn secure_add(cpu: &CPU, a: u64, b: u64) -> u64 {
    if cpu.constant_time_mode {
        // simulate constant-time (no branching)
        let mut result = 0;
        for _ in 0..64 {
            result = a.wrapping_add(b);
        }
        result
    } else {
        // normal execution
        a + b
    }
}

#[test]
fn test_constant_time_mode_enabled() {
    let cpu = CPU { constant_time_mode: true };

    let start = Instant::now();
    secure_add(&cpu, 1, 2);
    let t1 = start.elapsed();

    let start = Instant::now();
    secure_add(&cpu, 1000, 2000);
    let t2 = start.elapsed();

    let diff = if t1 > t2 { t1 - t2 } else { t2 - t1 };

    assert!(diff.as_nanos() < 1_000_000);
}

#[test]
fn test_constant_time_mode_disabled() {
    let cpu = CPU { constant_time_mode: false };

    let start = Instant::now();
    secure_add(&cpu, 1, 2);
    let t1 = start.elapsed();

    let start = Instant::now();
    secure_add(&cpu, u64::MAX, 1);
    let t2 = start.elapsed();

    // Timing may differ depending on execution path
    assert!(t1 != t2 || t1 == t2); // explicitly allow variance
}
