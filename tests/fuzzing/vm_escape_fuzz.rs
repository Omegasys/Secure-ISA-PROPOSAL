use rand::{Rng, thread_rng};

#[derive(Clone)]
struct VM {
    id: u32,
}

fn can_access(vm: &VM, target_id: u32) -> bool {
    vm.id == target_id
}

#[test]
fn fuzz_vm_escape() {
    let mut rng = thread_rng();

    for _ in 0..10000 {
        let vm_id = rng.gen::<u32>();
        let target_id = rng.gen::<u32>();

        let vm = VM { id: vm_id };

        let allowed = can_access(&vm, target_id);

        if vm_id != target_id {
            assert!(!allowed);
        }
    }
}
