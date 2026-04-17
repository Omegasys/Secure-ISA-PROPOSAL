use rand::{Rng, thread_rng};

fn access(memory: &[u8], addr: usize) -> Option<u8> {
    memory.get(addr).copied()
}

#[test]
fn fuzz_memory_access() {
    let mut rng = thread_rng();
    let memory = vec![0u8; 1024];

    for _ in 0..10000 {
        let addr = rng.gen::<usize>();

        let result = access(&memory, addr);

        if addr >= memory.len() {
            assert!(result.is_none());
        } else {
            assert!(result.is_some());
        }
    }
}
