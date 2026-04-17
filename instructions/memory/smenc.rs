use crate::cpu::Cpu;
use crate::memory::{Memory, MemoryRegion};
use crate::instructions::memory::Instruction;

pub enum CryptoMode {
    Encrypt,
    Decrypt,
}

pub struct SMemEnc {
    pub addr: usize,
    pub len: usize,
    pub mode: CryptoMode,
}

impl Instruction for SMemEnc {
    fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) -> Result<(), String> {
        // Require privilege for region-wide encryption
        if !cpu.is_privileged() {
            return Err("INSUFFICIENT_PRIVILEGE".into());
        }

        for addr in self.addr..(self.addr + self.len) {
            let region = memory
                .regions
                .get_mut(&addr)
                .ok_or("INVALID_ADDRESS")?;

            match self.mode {
                CryptoMode::Encrypt => {
                    region.encrypted = true;
                }
                CryptoMode::Decrypt => {
                    region.encrypted = false;
                }
            }
        }

        cpu.audit("SMENC", self.addr);

        Ok(())
    }
}