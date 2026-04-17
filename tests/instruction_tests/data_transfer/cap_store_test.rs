use crate::core::memory::Capability;

#[test]
fn test_capability_store_and_modify() {
    let mut cap = Capability {
        base: 0,
        limit: 50,
        permissions: 0b0011,
    };

    // simulate capability update (e.g., delegation)
    cap.limit = 25;

    assert_eq!(cap.limit, 25);
}

#[test]
fn test_capability_permission_restriction() {
    let cap = Capability {
        base: 0,
        limit: 50,
        permissions: 0b0001, // READ only
    };

    let can_write = cap.permissions & 0b0010 != 0;

    assert!(!can_write);
}
