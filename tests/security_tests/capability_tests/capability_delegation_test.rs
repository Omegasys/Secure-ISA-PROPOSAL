use crate::core::memory::Capability;

fn delegate(parent: &Capability, new_limit: u64) -> Option<Capability> {
    if new_limit <= parent.limit {
        Some(Capability {
            base: parent.base,
            limit: new_limit,
            permissions: parent.permissions,
        })
    } else {
        None
    }
}

#[test]
fn test_valid_delegation() {
    let parent = Capability {
        base: 0,
        limit: 100,
        permissions: 0b0011,
    };

    let child = delegate(&parent, 50);

    assert!(child.is_some());
    assert_eq!(child.unwrap().limit, 50);
}

#[test]
fn test_invalid_delegation_expand() {
    let parent = Capability {
        base: 0,
        limit: 100,
        permissions: 0b0011,
    };

    let child = delegate(&parent, 150);

    assert!(child.is_none());
}
