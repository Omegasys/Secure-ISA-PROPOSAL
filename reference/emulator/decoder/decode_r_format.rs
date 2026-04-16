pub struct RInstr {
    pub opcode: u32,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}

pub fn decode_r(inst: u32) -> RInstr {
    RInstr {
        opcode: inst & 0x7F,
        rd: ((inst >> 7) & 0x1F) as u8,
        rs1: ((inst >> 15) & 0x1F) as u8,
        rs2: ((inst >> 20) & 0x1F) as u8,
    }
}
