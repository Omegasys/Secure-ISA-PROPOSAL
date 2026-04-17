use rand::{Rng, thread_rng};
use std::collections::HashSet;

#[derive(Default)]
struct Sandbox {
    allowed: HashSet<u32>,
}

fn syscall(sb: &Sandbox, id: u32) -> bool {
    sb.allowed.contains(&id)
}

#[test]
fn fuzz_syscalls() {
    let mut rng = thread_rng();

    for _ in 0..10000 {
        let mut sb = Sandbox::default();

        let allowed_id = rng.gen::<u32>();
        sb.allowed.insert(allowed_id);

        let test_id = rng.gen::<u32>();

        let result = syscall(&sb, test_id);

        if test_id == allowed_id {
            assert!(result);
        }
    }
}
