use std::collections::HashSet;

#[derive(Clone)]
struct Sandbox {
    allowed_syscalls: HashSet<u32>,
}

fn syscall_allowed(sb: &Sandbox, syscall_id: u32) -> bool {
    sb.allowed_syscalls.contains(&syscall_id)
}

#[test]
fn test_syscall_allowed() {
    let mut allowed = HashSet::new();
    allowed.insert(1);

    let sb = Sandbox { allowed_syscalls: allowed };

    assert!(syscall_allowed(&sb, 1));
}

#[test]
fn test_syscall_denied() {
    let sb = Sandbox {
        allowed_syscalls: HashSet::new(),
    };

    assert!(!syscall_allowed(&sb, 99));
}
