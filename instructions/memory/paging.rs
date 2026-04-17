pub struct PageTableEntry {
    pub physical: usize,
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub encrypted: bool,
}

pub struct PageTable {
    pub entries: HashMap<usize, PageTableEntry>,
}

impl PageTable {
    pub fn translate(&self, virtual_addr: usize) -> Result<&PageTableEntry, String> {
        self.entries.get(&virtual_addr).ok_or("PAGE_FAULT".into())
    }
}