#[derive(Clone)]
struct Sandbox {
    id: u32,
    mem_base: u64,
    mem_limit: u64,
}

fn can_access(sb: &Sandbox, addr: u64) -> bool {
    addr >= sb.mem_base && addr < sb.mem_limit
}

#[test]
fn test_cross_sandbox_access_denied() {
    let sb1 = Sandbox { id: 1, mem_base: 0, mem_limit: 100 };
    let sb2 = Sandbox { id: 2, mem_base: 100, mem_limit: 200 };

    // sb1 tries to access sb2 memory
    let allowed = can_access(&sb1, 150);

    assert!(!allowed);
}

#[test]
fn test_isolated_regions() {
    let sb1 = Sandbox { id: 1, mem_base: 0, mem_limit: 100 };
    let sb2 = Sandbox { id: 2, mem_base: 100, mem_limit: 200 };

    assert!(can_access(&sb1, 50));
    assert!(can_access(&sb2, 150));
}
