use std::collections::HashMap;

pub struct PageTableEntry {
    pub physical: usize,
    pub read: bool,
    pub write: bool,
}

pub struct PageTable {
    pub entries: HashMap<usize, PageTableEntry>,
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn map(&mut self, vaddr: usize, paddr: usize) {
        self.entries.insert(vaddr, PageTableEntry {
            physical: paddr,
            read: true,
            write: true,
        });
    }

    pub fn translate(&self, vaddr: usize) -> Result<&PageTableEntry, String> {
        self.entries.get(&vaddr).ok_or("PAGE_FAULT".into())
    }
}