use crate::core::state::SystemState;

pub fn enter_sandbox(state: &mut SystemState) {
    state.cpu.privilege_level = 1;
}

pub fn exit_sandbox(state: &mut SystemState) {
    state.cpu.privilege_level = 0;
}

pub fn enforce_io_restriction(allow_io: bool) -> bool {
    allow_io
}

pub fn enforce_memory_restriction(allow_shared_memory: bool) -> bool {
    allow_shared_memory
}
