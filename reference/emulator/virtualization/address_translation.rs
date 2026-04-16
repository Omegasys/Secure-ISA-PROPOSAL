pub fn translate_address(guest_addr: u64, offset: u64) -> u64 {
    guest_addr.wrapping_add(offset)
}

pub fn isolate_address_space(addr: u64, vm_id: u64) -> u64 {
    (vm_id << 48) | (addr & 0x0000FFFFFFFFFFFF)
}
