use rand::{Rng, thread_rng};

#[derive(Clone)]
struct Sandbox {
    base: u64,
    limit: u64,
}

fn access(sb: &Sandbox, addr: u64) -> bool {
    addr >= sb.base && addr < sb.limit
}

#[test]
fn fuzz_sandbox_escape() {
    let mut rng = thread_rng();

    for _ in 0..10000 {
        let base = rng.gen::<u64>();
        let limit = base.wrapping_add(rng.gen_range(0..1000));

        let sb = Sandbox { base, limit };

        let addr = rng.gen::<u64>();
        let allowed = access(&sb, addr);

        if addr < base || addr >= limit {
            assert!(!allowed);
        }
    }
}
