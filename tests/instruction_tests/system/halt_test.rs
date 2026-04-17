use crate::core::state::SystemState;

fn halt(state: &mut SystemState) {
    state.cpu.halted = true;
}

#[test]
fn test_halt() {
    let mut state = SystemState::new();

    halt(&mut state);

    assert!(state.cpu.halted);
}
