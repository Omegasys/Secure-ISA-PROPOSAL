use std::collections::HashMap;

#[derive(Clone)]
struct VM {
    id: u32,
}

fn destroy_vm(vms: &mut HashMap<u32, VM>, id: u32) -> bool {
    vms.remove(&id).is_some()
}

#[test]
fn test_vm_teardown() {
    let mut vms = HashMap::new();
    vms.insert(1, VM { id: 1 });

    let result = destroy_vm(&mut vms, 1);

    assert!(result);
    assert!(!vms.contains_key(&1));
}

#[test]
fn test_vm_teardown_nonexistent() {
    let mut vms = HashMap::new();

    let result = destroy_vm(&mut vms, 1);

    assert!(!result);
}
