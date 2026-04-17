use crate::core::state::SystemState;

fn ret(state: &mut SystemState, return_reg: usize) {
    state.cpu.pc = state.cpu.registers[return_reg];
}

#[test]
fn test_return_basic() {
    let mut state = SystemState::new();

    state.cpu.registers[1] = 150;

    ret(&mut state, 1);

    assert_eq!(state.cpu.pc, 150);
}

#[test]
fn test_return_chain() {
    let mut state = SystemState::new();

    state.cpu.registers[2] = 500;

    ret(&mut state, 2);

    assert_eq!(state.cpu.pc, 500);
}
