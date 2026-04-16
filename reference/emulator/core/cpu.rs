pub struct CPU {
    pub pc: u64,
    pub registers: [u128; 32],
    pub fp_registers: [u128; 32],

    pub flags: u64,

    pub privilege_level: u8,
    pub user_level: u8,
    pub hypervisor_level: u8,
    pub os_level: u8,

    pub halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            registers: [0; 32],
            fp_registers: [0; 32],
            flags: 0,
            privilege_level: 0,
            user_level: 0,
            hypervisor_level: 0,
            os_level: 0,
            halted: false,
        }
    }

    pub fn set_flag(&mut self, flag: u64) {
        self.flags |= flag;
    }

    pub fn clear_flag(&mut self, flag: u64) {
        self.flags &= !flag;
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }
}
