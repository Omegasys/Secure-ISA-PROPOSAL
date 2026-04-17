use rand::{Rng, thread_rng};

#[derive(Clone)]
struct Capability {
    base: u64,
    limit: u64,
}

fn valid(cap: &Capability, addr: u64) -> bool {
    addr >= cap.base && addr < cap.limit
}

#[test]
fn fuzz_capabilities() {
    let mut rng = thread_rng();

    for _ in 0..10000 {
        let base = rng.gen::<u64>();
        let limit = rng.gen::<u64>();
        let addr = rng.gen::<u64>();

        let cap = Capability { base, limit };

        let result = valid(&cap, addr);

        // invariant: if base >= limit → always false
        if base >= limit {
            assert!(!result);
        }
    }
}
