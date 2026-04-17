#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
enum PrivilegeLevel {
    User = 0,
    Program = 1,
    OS = 2,
    Hypervisor = 3,
}

fn attempt_escalation(current: PrivilegeLevel, target: PrivilegeLevel) -> bool {
    target <= current // only allow equal or lower privilege
}

#[test]
fn test_privilege_escalation_denied() {
    let current = PrivilegeLevel::User;
    let target = PrivilegeLevel::OS;

    let allowed = attempt_escalation(current, target);

    assert!(!allowed);
}

#[test]
fn test_same_level_allowed() {
    let current = PrivilegeLevel::Program;

    assert!(attempt_escalation(current, current));
}
