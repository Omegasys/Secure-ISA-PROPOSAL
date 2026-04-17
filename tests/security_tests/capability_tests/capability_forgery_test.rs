use crate::core::memory::Capability;

fn is_valid_capability(cap: &Capability, system_max: u64) -> bool {
    cap.base < cap.limit && cap.limit <= system_max
}

#[test]
fn test_valid_capability() {
    let cap = Capability {
        base: 0,
        limit: 100,
        permissions: 0b0011,
    };

    assert!(is_valid_capability(&cap, 1000));
}

#[test]
fn test_forged_capability_out_of_range() {
    let cap = Capability {
        base: 0,
        limit: 10_000, // exceeds system bounds
        permissions: 0b0011,
    };

    assert!(!is_valid_capability(&cap, 1000));
}

#[test]
fn test_forged_capability_invalid_bounds() {
    let cap = Capability {
        base: 100,
        limit: 50,
        permissions: 0b0011,
    };

    assert!(!is_valid_capability(&cap, 1000));
}
