use rand::{Rng, thread_rng};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Level {
    User = 0,
    Program = 1,
    OS = 2,
    Hypervisor = 3,
}

fn can_escalate(current: Level, target: Level) -> bool {
    target <= current
}

#[test]
fn fuzz_privilege_escalation() {
    let mut rng = thread_rng();

    let levels = [
        Level::User,
        Level::Program,
        Level::OS,
        Level::Hypervisor,
    ];

    for _ in 0..10000 {
        let current = levels[rng.gen_range(0..levels.len())];
        let target = levels[rng.gen_range(0..levels.len())];

        let allowed = can_escalate(current, target);

        if target > current {
            assert!(!allowed);
        }
    }
}
