use crate::core::memory::Capability;

fn within_bounds(cap: &Capability, addr: u64) -> bool {
    addr >= cap.base && addr < cap.limit
}

#[test]
fn test_within_bounds_valid() {
    let cap = Capability {
        base: 10,
        limit: 20,
        permissions: 0b0001,
    };

    assert!(within_bounds(&cap, 15));
}

#[test]
fn test_out_of_bounds_low() {
    let cap = Capability {
        base: 10,
        limit: 20,
        permissions: 0b0001,
    };

    assert!(!within_bounds(&cap, 5));
}

#[test]
fn test_out_of_bounds_high() {
    let cap = Capability {
        base: 10,
        limit: 20,
        permissions: 0b0001,
    };

    assert!(!within_bounds(&cap, 25));
}
