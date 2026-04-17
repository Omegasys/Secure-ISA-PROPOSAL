#[derive(Clone)]
struct VM {
    id: u32,
}

fn can_vm_access(vm_id: u32, target_vm_id: u32) -> bool {
    vm_id == target_vm_id
}

#[test]
fn test_vm_self_access() {
    assert!(can_vm_access(1, 1));
}

#[test]
fn test_vm_cross_access_denied() {
    assert!(!can_vm_access(1, 2));
}
