use crate::memory::Memory;
use crate::paging::PageTable;
use crate::cpu::privilege::PrivilegeContext;
use crate::cpu::audit::AuditLog;
use crate::cpu::crypto::CryptoEngine;

pub struct Cpu {
    pub registers: Vec<u8>,
    pub privilege: PrivilegeContext,
    pub audit: AuditLog,
    pub crypto: CryptoEngine,
    pub page_table: PageTable,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: vec![0; 32],
            privilege: PrivilegeContext {
                user_level: 0,
                program_level: 0,
                os_level: 0,
                hypervisor_level: 0,
            },
            audit: AuditLog::new(),
            crypto: CryptoEngine::new(),
            page_table: PageTable::new(),
        }
    }

    pub fn check_access(&self, required: &PrivilegeContext) -> bool {
        self.privilege.can_access(required)
    }

    pub fn is_privileged(&self) -> bool {
        self.privilege.os_level > 0 || self.privilege.hypervisor_level > 0
    }

    pub fn audit(&mut self, instr: &str, addr: usize) {
        self.audit.record(format!("[AUDIT] {} at {}", instr, addr));
    }

    pub fn encrypt(&self, data: u8) -> u8 {
        self.crypto.encrypt(data)
    }

    pub fn decrypt(&self, data: u8) -> u8 {
        self.crypto.decrypt(data)
    }
}