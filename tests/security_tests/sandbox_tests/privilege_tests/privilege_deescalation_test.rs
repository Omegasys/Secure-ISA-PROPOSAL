#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum PrivilegeLevel {
    User = 0,
    Program = 1,
    OS = 2,
    Hypervisor = 3,
}

fn deescalate(current: PrivilegeLevel, target: PrivilegeLevel) -> bool {
    target < current
}

#[test]
fn test_valid_deescalation() {
    let current = PrivilegeLevel::OS;
    let target = PrivilegeLevel::User;

    assert!(deescalate(current, target));
}

#[test]
fn test_invalid_deescalation() {
    let current = PrivilegeLevel::User;
    let target = PrivilegeLevel::OS;

    assert!(!deescalate(current, target));
}
