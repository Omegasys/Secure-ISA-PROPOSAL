use std::collections::HashMap;

pub type Address = u64;

#[derive(Clone)]
pub struct Capability {
    pub base: Address,
    pub limit: Address,
    pub permissions: u32,
}

pub struct Memory {
    pub data: HashMap<Address, u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn read(&self, addr: Address, cap: &Capability) -> Option<u8> {
        if addr < cap.base || addr >= cap.limit {
            return None;
        }
        self.data.get(&addr).copied()
    }

    pub fn write(&mut self, addr: Address, value: u8, cap: &Capability) -> Result<(), &'static str> {
        if addr < cap.base || addr >= cap.limit {
            return Err("CAPABILITY_VIOLATION");
        }

        self.data.insert(addr, value);
        Ok(())
    }
}
