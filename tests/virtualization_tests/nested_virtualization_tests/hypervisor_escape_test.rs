#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Level {
    VM = 0,
    H1 = 1,
    H2 = 2,
}

fn attempt_escape(current: Level, target: Level) -> bool {
    target <= current
}

#[test]
fn test_vm_escape_attempt() {
    let current = Level::VM;
    let target = Level::H1;

    let allowed = attempt_escape(current, target);

    assert!(!allowed);
}

#[test]
fn test_h1_cannot_escape_to_h2() {
    let current = Level::H1;
    let target = Level::H2;

    assert!(!allowed_escape(current, target));
}

// helper to make intent explicit
fn allowed_escape(current: Level, target: Level) -> bool {
    target <= current
}
