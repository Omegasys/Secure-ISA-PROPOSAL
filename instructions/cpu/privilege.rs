#[derive(Clone)]
pub struct PrivilegeContext {
    pub user_level: u8,        // 0–15
    pub program_level: u8,     // 0–5
    pub os_level: u8,          // 0–4
    pub hypervisor_level: u8,  // 0–4
}

impl PrivilegeContext {
    pub fn can_access(&self, required: &PrivilegeContext) -> bool {
        self.user_level >= required.user_level &&
        self.program_level >= required.program_level &&
        self.os_level >= required.os_level &&
        self.hypervisor_level >= required.hypervisor_level
    }
}