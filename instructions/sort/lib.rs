pub mod encoding;
pub mod opcode;

pub mod secure_move;
pub mod hw_accel;
pub mod power;
pub mod pcie;

/// ISA execution context (VM or hardware abstraction)
pub struct IsaContext {
    pub privilege_level: u8,
    pub hw_accel_enabled: bool,
}

impl IsaContext {
    pub fn new() -> Self {
        Self {
            privilege_level: 0,
            hw_accel_enabled: true,
        }
    }
}
