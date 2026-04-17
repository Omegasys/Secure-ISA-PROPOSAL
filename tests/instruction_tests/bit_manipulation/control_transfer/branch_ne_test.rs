use crate::core::state::SystemState;

fn branch_ne(state: &mut SystemState, a: u64, b: u64, target: u64) {
    if a != b {
        state.cpu.pc = target;
    }
}

#[test]
fn test_branch_ne_taken() {
    let mut state = SystemState::new();

    branch_ne(&mut state, 5, 6, 200);

    assert_eq!(state.cpu.pc, 200);
}

#[test]
fn test_branch_ne_not_taken() {
    let mut state = SystemState::new();
    state.cpu.pc = 20;

    branch_ne(&mut state, 7, 7, 200);

    assert_eq!(state.cpu.pc, 20);
}
