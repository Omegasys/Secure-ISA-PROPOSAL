use crate::instructions::memory::Instruction;
use crate::cpu::Cpu;
use crate::memory::Memory;

pub enum CryptoMode {
    Encrypt,
    Decrypt,
}

pub enum CryptoType {
    Symmetric,
    Asymmetric,
    Hybrid,
}

pub struct SMemEnc {
    pub addr: usize,
    pub len: usize,
    pub mode: CryptoMode,
    pub ctype: CryptoType,
}

impl Instruction for SMemEnc {
    fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) -> Result<(), String> {
        for i in 0..self.len {
            let mut data = memory.read(self.addr + i)?;

            data = match self.mode {
                CryptoMode::Encrypt => cpu.encrypt(data)?,
                CryptoMode::Decrypt => cpu.decrypt(data)?,
            };

            memory.write(self.addr + i, data)?;
        }

        cpu.audit("SMENC", self.addr);
        Ok(())
    }
}