#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Level {
    VM = 0,
    H1 = 1,
    H2 = 2,
}

fn can_control(parent: Level, child: Level) -> bool {
    parent > child
}

#[test]
fn test_h2_controls_h1_and_vm() {
    assert!(can_control(Level::H2, Level::H1));
    assert!(can_control(Level::H2, Level::VM));
}

#[test]
fn test_h1_cannot_control_h2() {
    assert!(!can_control(Level::H1, Level::H2));
}
