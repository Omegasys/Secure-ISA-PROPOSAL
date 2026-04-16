use crate::core::memory::Capability;

pub fn check_read(cap: &Capability) -> bool {
    cap.permissions & 0b0001 != 0
}

pub fn check_write(cap: &Capability) -> bool {
    cap.permissions & 0b0010 != 0
}

pub fn check_execute(cap: &Capability) -> bool {
    cap.permissions & 0b0100 != 0
}

pub fn check_delegate(cap: &Capability) -> bool {
    cap.permissions & 0b1000 != 0
}

pub fn validate_memory_access(cap: &Capability, addr: u64) -> bool {
    addr >= cap.base && addr < cap.limit
}
