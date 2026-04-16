use super::decode::*;
use super::format::*;

pub fn disassemble(inst: u32) -> String {
    let opcode = decode_opcode(inst);

    match opcode {
        0x10 => format_r("add", decode_rd(inst), decode_rs1(inst), decode_rs2(inst)),
        0x11 => format_r("sub", decode_rd(inst), decode_rs1(inst), decode_rs2(inst)),
        0xA0 => "halt".to_string(),
        _ => "unknown".to_string(),
    }
}
