use super::super::core::state::SystemState;
use super::super::core::memory::Capability;

pub fn load(state: &mut SystemState, addr: u64, cap: &Capability) -> Option<u8> {
    state.memory.read(addr, cap)
}

pub fn store(state: &mut SystemState, addr: u64, value: u8, cap: &Capability) -> Result<(), &'static str> {
    state.memory.write(addr, value, cap)
}
