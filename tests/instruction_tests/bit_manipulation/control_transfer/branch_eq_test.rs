use crate::core::state::SystemState;
use crate::execution::control_flow::branch_eq;

#[test]
fn test_branch_eq_taken() {
    let mut state = SystemState::new();

    branch_eq(&mut state, 5, 5, 100);

    assert_eq!(state.cpu.pc, 100);
}

#[test]
fn test_branch_eq_not_taken() {
    let mut state = SystemState::new();
    state.cpu.pc = 10;

    branch_eq(&mut state, 5, 6, 100);

    assert_eq!(state.cpu.pc, 10);
}
