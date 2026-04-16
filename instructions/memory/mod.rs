pub mod load;
pub mod store;
pub mod memcpy;
pub mod memprotect;
pub mod memtag;
pub mod memencrypt;
pub mod memaudit;
pub mod memimmutable;

use crate::cpu::Cpu;
use crate::memory::Memory;

pub trait Instruction {
    fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) -> Result<(), String>;
}