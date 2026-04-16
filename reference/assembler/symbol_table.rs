use std::collections::HashMap;

pub struct SymbolTable {
    pub labels: HashMap<String, u64>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
        }
    }

    pub fn insert(&mut self, label: &str, addr: u64) {
        self.labels.insert(label.to_string(), addr);
    }

    pub fn get(&self, label: &str) -> Option<&u64> {
        self.labels.get(label)
    }
}
