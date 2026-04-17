use std::collections::HashMap;

pub struct MemoryRegion {
    pub data: u8,
    pub encrypted: bool,
}

pub struct Memory {
    pub regions: HashMap<usize, MemoryRegion>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
        }
    }

    pub fn read(&self, addr: usize) -> Result<u8, String> {
        let region = self.regions.get(&addr).ok_or("INVALID_ADDR")?;

        let mut data = region.data;

        if region.encrypted {
            data ^= 0xAA;
        }

        Ok(data)
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        self.regions.insert(addr, MemoryRegion {
            data: value,
            encrypted: false,
        });
    }
}