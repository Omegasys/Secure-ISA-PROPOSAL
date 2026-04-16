use super::super::core::state::SystemState;

pub fn syscall(state: &mut SystemState, id: u32) {
    match id {
        0 => state.cpu.halted = true,
        _ => {}
    }
}

pub fn enter_secure(state: &mut SystemState) {
    state.cpu.privilege_level = 255;
}

pub fn exit_secure(state: &mut SystemState) {
    state.cpu.privilege_level = 0;
}
