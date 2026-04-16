#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    User = 0,
    System = 1,
    OS = 2,
    Hypervisor = 3,
    Kernel = 4,
}

pub fn can_escalate(from: PrivilegeLevel, to: PrivilegeLevel) -> bool {
    (from as u8) < (to as u8)
}

pub fn enforce_transition(from: PrivilegeLevel, to: PrivilegeLevel) -> bool {
    match to {
        PrivilegeLevel::Kernel => from == PrivilegeLevel::Hypervisor,
        _ => can_escalate(from, to),
    }
}
