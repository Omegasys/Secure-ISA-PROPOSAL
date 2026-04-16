use super::super::core::state::SystemState;

pub fn jump(state: &mut SystemState, target: u64) {
    state.cpu.pc = target;
}

pub fn halt(state: &mut SystemState) {
    state.cpu.halted = true;
}

pub fn branch_eq(state: &mut SystemState, a: u64, b: u64, target: u64) {
    if a == b {
        state.cpu.pc = target;
    }
}
