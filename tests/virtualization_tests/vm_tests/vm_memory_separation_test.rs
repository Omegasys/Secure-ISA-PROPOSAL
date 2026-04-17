#[derive(Clone)]
struct VM {
    mem_base: u64,
    mem_limit: u64,
}

fn can_access(vm: &VM, addr: u64) -> bool {
    addr >= vm.mem_base && addr < vm.mem_limit
}

#[test]
fn test_vm_memory_isolated() {
    let vm1 = VM { mem_base: 0, mem_limit: 100 };
    let vm2 = VM { mem_base: 100, mem_limit: 200 };

    assert!(can_access(&vm1, 50));
    assert!(!can_access(&vm1, 150)); // vm2 memory
}
