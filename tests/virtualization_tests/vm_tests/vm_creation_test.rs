#[derive(Clone)]
struct VM {
    id: u32,
    mem_base: u64,
    mem_limit: u64,
}

fn create_vm(id: u32, base: u64, limit: u64) -> Option<VM> {
    if base >= limit {
        return None;
    }

    Some(VM { id, mem_base: base, mem_limit: limit })
}

#[test]
fn test_vm_creation_valid() {
    let vm = create_vm(1, 0, 1024);

    assert!(vm.is_some());
}

#[test]
fn test_vm_creation_invalid_bounds() {
    let vm = create_vm(1, 100, 50);

    assert!(vm.is_none());
}
