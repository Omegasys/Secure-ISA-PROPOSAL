pub fn decode_opcode(inst: u32) -> u32 {
    inst & 0x7F
}

pub fn decode_rd(inst: u32) -> u8 {
    ((inst >> 7) & 0x1F) as u8
}

pub fn decode_rs1(inst: u32) -> u8 {
    ((inst >> 15) & 0x1F) as u8
}

pub fn decode_rs2(inst: u32) -> u8 {
    ((inst >> 20) & 0x1F) as u8
}
