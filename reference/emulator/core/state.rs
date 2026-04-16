use super::cpu::CPU;
use super::memory::Memory;

pub struct SystemState {
    pub cpu: CPU,
    pub memory: Memory,
}

impl SystemState {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            memory: Memory::new(),
        }
    }
}
