use crate::instructions::memory::Instruction;
use crate::cpu::Cpu;
use crate::memory::Memory;

pub enum AuditMode {
    Enable,
    Disable,
    Query,
}

pub struct SMemAudit {
    pub addr: usize,
    pub len: usize,
    pub mode: AuditMode,
}

impl Instruction for SMemAudit {
    fn execute(&self, cpu: &mut Cpu, _memory: &mut Memory) -> Result<(), String> {
        match self.mode {
            AuditMode::Enable => cpu.enable_audit(self.addr, self.len),
            AuditMode::Disable => cpu.disable_audit(self.addr, self.len),
            AuditMode::Query => cpu.query_audit(self.addr, self.len),
        }

        Ok(())
    }
}