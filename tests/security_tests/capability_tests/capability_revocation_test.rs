use crate::core::memory::Capability;

#[derive(Clone)]
struct RevocableCapability {
    cap: Capability,
    revoked: bool,
}

fn revoke(cap: &mut RevocableCapability) {
    cap.revoked = true;
}

fn is_valid(cap: &RevocableCapability) -> bool {
    !cap.revoked
}

#[test]
fn test_revocation() {
    let mut cap = RevocableCapability {
        cap: Capability {
            base: 0,
            limit: 100,
            permissions: 0b0011,
        },
        revoked: false,
    };

    revoke(&mut cap);

    assert!(!is_valid(&cap));
}

#[test]
fn test_use_after_revocation() {
    let mut cap = RevocableCapability {
        cap: Capability {
            base: 0,
            limit: 100,
            permissions: 0b0011,
        },
        revoked: false,
    };

    revoke(&mut cap);

    let allowed = is_valid(&cap);

    assert!(!allowed); // should block usage
}
