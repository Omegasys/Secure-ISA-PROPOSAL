#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Level {
    VM = 0,
    Hypervisor1 = 1,
}

fn can_control(parent: Level, child: Level) -> bool {
    parent > child
}

#[test]
fn test_level1_controls_vm() {
    assert!(can_control(Level::Hypervisor1, Level::VM));
}

#[test]
fn test_vm_cannot_control_hypervisor() {
    assert!(!can_control(Level::VM, Level::Hypervisor1));
}
