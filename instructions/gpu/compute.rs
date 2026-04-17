use crate::gpu::memory::GpuMemory;

pub struct Gpu;

impl Gpu {
    pub fn execute(memory: &mut GpuMemory, id: u32) -> Result<(), String> {
        let buffer = memory.buffers.get_mut(&id).ok_or("INVALID_BUFFER")?;

        // Simple example compute: increment all values
        for val in buffer.iter_mut() {
            *val += 1;
        }

        Ok(())
    }
}