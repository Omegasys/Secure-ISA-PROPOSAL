use crate::core::memory::Capability;

#[test]
fn test_capability_load_fields() {
    let cap = Capability {
        base: 100,
        limit: 200,
        permissions: 0b1111,
    };

    assert_eq!(cap.base, 100);
    assert_eq!(cap.limit, 200);
    assert_eq!(cap.permissions, 0b1111);
}
