use crate::instructions::memory::Instruction;
use crate::cpu::Cpu;
use crate::memory::{Memory, Permissions};

pub struct SMemProtect {
    pub addr: usize,
    pub len: usize,
    pub perms: Permissions,
}

impl Instruction for SMemProtect {
    fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) -> Result<(), String> {
        if !cpu.is_privileged() {
            return Err("INSUFFICIENT_PRIVILEGE".into());
        }

        memory.set_permissions(self.addr, self.len, self.perms)?;
        cpu.audit("SMPROTECT", self.addr);

        Ok(())
    }
}