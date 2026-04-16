pub fn format_r(op: &str, rd: u8, rs1: u8, rs2: u8) -> String {
    format!("{} r{}, r{}, r{}", op, rd, rs1, rs2)
}

pub fn format_s(op: &str, a: u8, b: u8, imm: u32) -> String {
    format!("{} r{}, r{}, #{}", op, a, b, imm)
}
