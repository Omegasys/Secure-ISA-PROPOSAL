pub struct SInstr {
    pub opcode: u32,
    pub a: u8,
    pub b: u8,
    pub imm: u32,
}

pub fn decode_s(inst: u32) -> SInstr {
    SInstr {
        opcode: inst & 0x7F,
        a: ((inst >> 7) & 0x1F) as u8,
        b: ((inst >> 15) & 0x1F) as u8,
        imm: (inst >> 20) & 0xFFF,
    }
}
