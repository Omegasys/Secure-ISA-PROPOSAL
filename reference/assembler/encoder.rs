pub fn encode_r(opcode: u32, rd: u8, rs1: u8, rs2: u8) -> u32 {
    (opcode)
        | ((rd as u32) << 7)
        | ((rs1 as u32) << 15)
        | ((rs2 as u32) << 20)
}

pub fn encode_s(opcode: u32, a: u8, b: u8, imm: u32) -> u32 {
    opcode
        | ((a as u32) << 7)
        | ((b as u32) << 15)
        | (imm << 20)
}
