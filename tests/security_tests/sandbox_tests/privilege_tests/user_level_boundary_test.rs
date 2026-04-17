#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum PrivilegeLevel {
    User = 0,
    Program = 1,
    OS = 2,
}

fn can_access_os_resource(level: PrivilegeLevel) -> bool {
    level >= PrivilegeLevel::OS
}

#[test]
fn test_user_access_denied() {
    assert!(!can_access_os_resource(PrivilegeLevel::User));
}

#[test]
fn test_os_access_allowed() {
    assert!(can_access_os_resource(PrivilegeLevel::OS));
}
