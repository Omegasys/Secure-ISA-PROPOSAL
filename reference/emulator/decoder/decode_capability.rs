use super::super::core::memory::Capability;

pub fn decode_cap(raw: u128) -> Capability {
    Capability {
        base: (raw & 0xFFFFFFFF) as u64,
        limit: ((raw >> 32) & 0xFFFFFFFF) as u64,
        permissions: ((raw >> 64) & 0xFFFFFFFF) as u32,
    }
}
