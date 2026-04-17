use crate::core::state::SystemState;
use crate::execution::control_flow::jump;

#[test]
fn test_jump_basic() {
    let mut state = SystemState::new();

    jump(&mut state, 100);

    assert_eq!(state.cpu.pc, 100);
}

#[test]
fn test_jump_overwrite_pc() {
    let mut state = SystemState::new();
    state.cpu.pc = 10;

    jump(&mut state, 200);

    assert_eq!(state.cpu.pc, 200);
}
