use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::paging::PageTable;
use crate::instructions::memory::Instruction;

pub enum LoadMode {
    Raw,
    Decrypt,
    Verify,
}

pub struct SLoad {
    pub dest: usize,
    pub vaddr: usize,
    pub mode: LoadMode,
}

impl Instruction for SLoad {
    fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) -> Result<(), String> {
        // 1. Translate virtual → physical
        let entry = cpu.page_table.translate(self.vaddr)?;

        if !entry.read {
            return Err("READ_VIOLATION".into());
        }

        // 2. Multi-level security check
        if !cpu.check_access(self.vaddr) {
            return Err("ACCESS_VIOLATION".into());
        }

        // 3. Read memory (auto-decrypt if encrypted)
        let mut data = memory.read(entry.physical)?;

        // 4. Optional instruction-level crypto
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

        // 5. Store in register
        cpu.registers[self.dest] = data;

        // 6. Audit
        cpu.audit("SLOAD", self.vaddr);

        Ok(())
    }
}