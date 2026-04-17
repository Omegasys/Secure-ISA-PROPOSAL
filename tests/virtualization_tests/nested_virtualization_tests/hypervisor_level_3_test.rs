#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Level {
    VM = 0,
    H1 = 1,
    H2 = 2,
    H3 = 3,
}

fn can_control(parent: Level, child: Level) -> bool {
    parent > child
}

#[test]
fn test_h3_full_control() {
    assert!(can_control(Level::H3, Level::H2));
    assert!(can_control(Level::H3, Level::H1));
    assert!(can_control(Level::H3, Level::VM));
}

#[test]
fn test_lower_levels_blocked() {
    assert!(!can_control(Level::VM, Level::H3));
}
