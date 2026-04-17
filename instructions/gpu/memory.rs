use std::collections::HashMap;

pub struct GpuMemory {
    pub buffers: HashMap<u32, Vec<u8>>,
}

impl GpuMemory {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
        }
    }

    pub fn alloc(&mut self, id: u32, size: usize) {
        self.buffers.insert(id, vec![0; size]);
    }

    pub fn free(&mut self, id: u32) {
        self.buffers.remove(&id);
    }
}