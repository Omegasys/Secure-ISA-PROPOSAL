use crate::opcode::Opcode;

/// Basic instruction format:
/// [opcode:1][flags:1][operand_a:4][operand_b:4]
#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub flags: u8,
    pub operand_a: u32,
    pub operand_b: u32,
}

impl Instruction {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.push(self.opcode as u8);
        out.push(self.flags);
        out.extend_from_slice(&self.operand_a.to_le_bytes());
        out.extend_from_slice(&self.operand_b.to_le_bytes());

        out
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 10 {
            return None;
        }

        Some(Self {
            opcode: unsafe { std::mem::transmute(bytes[0]) },
            flags: bytes[1],
            operand_a: u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
            operand_b: u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]),
        })
    }
}
