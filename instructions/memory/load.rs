use crate::instructions::memory::Instruction;
use crate::cpu::Cpu;
use crate::memory::Memory;

pub enum LoadMode {
    Raw,
    Decrypt,
    Verify,
}

pub struct SLoad {
    pub dest: usize,
    pub addr: usize,
    pub mode: LoadMode,
}

impl Instruction for SLoad {
    fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) -> Result<(), String> {
        // 1. Permission check (multi-layer)
        if !cpu.check_access(self.addr) {
            return Err("ACCESS_VIOLATION".into());
        }

        // 2. Read memory
        let mut data = memory.read(self.addr)?;

        // 3. Apply mode
        match self.mode {
            LoadMode::Raw => {}
            LoadMode::Decrypt => {
                data = cpu.decrypt(data)?;
            }
            LoadMode::Verify => {
                data = cpu.decrypt(data)?;
                cpu.verify_integrity(data)?;
            }
        }

        // 4. Store in register
        cpu.registers[self.dest] = data;

        // 5. Audit log
        cpu.audit("SLOAD", self.addr);

        Ok(())
    }
}