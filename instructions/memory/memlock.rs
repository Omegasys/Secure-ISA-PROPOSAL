use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::instructions::memory::Instruction;

/// Locks or unlocks memory using a password-derived key
pub struct SMemLock {
    pub password: String,
}

impl Instruction for SMemLock {
    fn execute(&self, cpu: &mut Cpu, memory: &mut Memory) -> Result<(), String> {
        // Only high privilege can unlock memory
        if !cpu.is_privileged() {
            return Err("INSUFFICIENT_PRIVILEGE".into());
        }

        // Derive key from password (placeholder)
        let key = derive_key(&self.password);

        memory.set_key(key);

        cpu.audit("SMEMLOCK", 0);

        Ok(())
    }
}

/// VERY simplified key derivation (replace with Argon2 later)
fn derive_key(password: &str) -> Vec<u8> {
    let mut key = vec![0u8; 32];

    for (i, byte) in password.bytes().enumerate() {
        key[i % 32] ^= byte;
    }

    key
}