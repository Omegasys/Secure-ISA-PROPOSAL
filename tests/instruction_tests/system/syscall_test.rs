use crate::core::state::SystemState;

fn syscall(state: &mut SystemState, syscall_id: u32) -> Result<(), &'static str> {
    match syscall_id {
        0 => Ok(()), // no-op syscall
        _ => Err("INVALID_SYSCALL"),
    }
}

#[test]
fn test_syscall_valid() {
    let mut state = SystemState::new();

    let result = syscall(&mut state, 0);

    assert!(result.is_ok());
}

#[test]
fn test_syscall_invalid() {
    let mut state = SystemState::new();

    let result = syscall(&mut state, 999);

    assert!(result.is_err());
}
