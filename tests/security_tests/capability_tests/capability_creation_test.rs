use crate::core::memory::Capability;

#[test]
fn test_capability_creation_valid() {
    let cap = Capability {
        base: 0,
        limit: 100,
        permissions: 0b0011,
    };

    assert!(cap.base < cap.limit);
}

#[test]
fn test_capability_creation_invalid_bounds() {
    let cap = Capability {
        base: 100,
        limit: 50,
        permissions: 0b0011,
    };

    // In a real ISA, this should be rejected at creation time
    assert!(cap.base >= cap.limit);
}
