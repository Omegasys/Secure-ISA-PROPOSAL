use std::collections::HashMap;

pub struct MemoryRegion {
    pub data: Vec<u8>,
    pub encrypted: bool,
    pub immutable: bool,
}

pub struct Memory {
    pub regions: HashMap<usize, MemoryRegion>,
    pub key: Option<Vec<u8>>, // derived from password
}

impl Memory {
    pub fn unlock(&mut self, password: &str) {
        // VERY simplified — you'd use Argon2 or similar
        self.key = Some(password.as_bytes().to_vec());
    }

    pub fn read(&self, addr: usize) -> Result<u8, String> {
        let region = self.regions.get(&addr).ok_or("INVALID_ADDR")?;

        let mut data = region.data[0];

        if region.encrypted {
            let key = self.key.as_ref().ok_or("MEMORY_LOCKED")?;
            data ^= key[0]; // placeholder encryption
        }

        Ok(data)
    }

    pub fn write(&mut self, addr: usize, value: u8) -> Result<(), String> {
        let region = self.regions.get_mut(&addr).ok_or("INVALID_ADDR")?;

        if region.immutable {
            return Err("IMMUTABLE_MEMORY".into());
        }

        let mut data = value;

        if region.encrypted {
            let key = self.key.as_ref().ok_or("MEMORY_LOCKED")?;
            data ^= key[0];
        }

        region.data[0] = data;
        Ok(())
    }
}

use std::collections::HashMap;

pub struct MemoryRegion {
    pub data: u8,
    pub encrypted: bool,
}

pub struct Memory {
    pub regions: HashMap<usize, MemoryRegion>,
    pub key: Option<Vec<u8>>,
}

impl Memory {
    pub fn set_key(&mut self, key: Vec<u8>) {
        self.key = Some(key);
    }

    pub fn read(&self, addr: usize) -> Result<u8, String> {
        let region = self.regions.get(&addr).ok_or("INVALID_ADDR")?;

        let mut data = region.data;

        if region.encrypted {
            let key = self.key.as_ref().ok_or("MEMORY_LOCKED")?;
            data ^= key[0]; // placeholder
        }

        Ok(data)
    }
}