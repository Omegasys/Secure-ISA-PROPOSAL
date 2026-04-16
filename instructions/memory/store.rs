use crate::instructions::memory::Instruction;
use crate::cpu::Cpu;
use crate::memory::Memory;

pub enum StoreMode {
    Raw,
    Encrypt,
    Sign,
}

pub struct SStore {
    pub addr: usize,
    pub src: usize,
    pub mode: StoreMode,
}

impl Instruction for SStore {
    fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) -> Result<(), String> {
        if !cpu.check_write(self.addr) {
            return Err("WRITE_VIOLATION".into());
        }

        let mut data = cpu.registers[self.src];

        match self.mode {
            StoreMode::Raw => {}
            StoreMode::Encrypt => {
                data = cpu.encrypt(data)?;
            }
            StoreMode::Sign => {
                data = cpu.encrypt(data)?;
                cpu.sign(data)?;
            }
        }

        memory.write(self.addr, data)?;
        cpu.audit("SSTORE", self.addr);

        Ok(())
    }
}