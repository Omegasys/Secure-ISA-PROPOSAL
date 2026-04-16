use std::collections::HashMap;

pub struct KeyStore {
    pub symmetric: HashMap<u64, Vec<u8>>,
    pub asymmetric: HashMap<u64, u64>,
}

impl KeyStore {
    pub fn new() -> Self {
        Self {
            symmetric: HashMap::new(),
            asymmetric: HashMap::new(),
        }
    }

    pub fn store_sym(&mut self, id: u64, key: Vec<u8>) {
        self.symmetric.insert(id, key);
    }

    pub fn store_asym(&mut self, id: u64, key: u64) {
        self.asymmetric.insert(id, key);
    }
}
