use crate::core::state::SystemState;

#[derive(Clone)]
struct Sandbox {
    mem_base: u64,
    mem_limit: u64,
}

fn access_memory(sb: &Sandbox, addr: u64) -> bool {
    addr >= sb.mem_base && addr < sb.mem_limit
}

#[test]
fn test_sandbox_escape_attempt() {
    let sb = Sandbox {
        mem_base: 0,
        mem_limit: 100,
    };

    let allowed = access_memory(&sb, 150); // outside sandbox

    assert!(!allowed);
}

#[test]
fn test_sandbox_valid_access() {
    let sb = Sandbox {
        mem_base: 0,
        mem_limit: 100,
    };

    assert!(access_memory(&sb, 50));
}
