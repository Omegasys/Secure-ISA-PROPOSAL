use crate::core::state::SystemState;

fn enter_secure(state: &mut SystemState) {
    state.cpu.secure_mode = true;
}

fn exit_secure(state: &mut SystemState) {
    state.cpu.secure_mode = false;
}

#[test]
fn test_enter_secure_mode() {
    let mut state = SystemState::new();

    enter_secure(&mut state);

    assert!(state.cpu.secure_mode);
}

#[test]
fn test_exit_secure_mode() {
    let mut state = SystemState::new();
    state.cpu.secure_mode = true;

    exit_secure(&mut state);

    assert!(!state.cpu.secure_mode);
}
