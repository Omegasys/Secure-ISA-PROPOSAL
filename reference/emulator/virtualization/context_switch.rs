use crate::core::state::SystemState;

pub fn save_state(state: &SystemState) -> SystemState {
    SystemState {
        cpu: state.cpu.clone(),
        memory: crate::core::memory::Memory::new(),
    }
}

pub fn restore_state(target: &mut SystemState, saved: SystemState) {
    target.cpu = saved.cpu;
}
