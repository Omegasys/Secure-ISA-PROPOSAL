#[derive(Clone)]
struct Capability {
    base: u64,
    limit: u64,
}

fn can_access(cap: &Capability, addr: u64) -> bool {
    addr >= cap.base && addr < cap.limit
}

fn delegate(cap: &Capability, new_limit: u64) -> Capability {
    Capability {
        base: cap.base,
        limit: new_limit.min(cap.limit),
    }
}

#[test]
fn test_capability_flow() {
    let root = Capability { base: 0, limit: 100 };

    // delegate reduced capability
    let child = delegate(&root, 50);

    // allowed within bounds
    assert!(can_access(&child, 25));

    // denied outside delegated bounds
    assert!(!can_access(&child, 75));
}
