#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum PrivilegeLevel {
    User = 0,
    Program = 1,
    OS = 2,
    Hypervisor = 3,
}

fn dominates(a: PrivilegeLevel, b: PrivilegeLevel) -> bool {
    a >= b
}

#[test]
fn test_lattice_ordering() {
    assert!(dominates(PrivilegeLevel::Hypervisor, PrivilegeLevel::User));
    assert!(dominates(PrivilegeLevel::OS, PrivilegeLevel::Program));
}

#[test]
fn test_lattice_violation() {
    assert!(!dominates(PrivilegeLevel::User, PrivilegeLevel::OS));
}
