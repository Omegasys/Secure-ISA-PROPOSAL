/// Axiom ISA opcodes (custom extension ISA layer)
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    SECURE_MOVE = 0x10,
    HW_ACCEL = 0x11,
    SLEEP = 0x20,
    HIBERNATE = 0x21,
    POWER_OFF = 0x22,
    PCIE_PASS_THROUGH = 0x30,
}
