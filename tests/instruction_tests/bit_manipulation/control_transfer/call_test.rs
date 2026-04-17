use crate::core::state::SystemState;

fn call(state: &mut SystemState, target: u64, return_reg: usize) {
    // store return address (next instruction)
    let return_addr = state.cpu.pc + 4;
    state.cpu.registers[return_reg] = return_addr;

    // jump to function
    state.cpu.pc = target;
}

#[test]
fn test_call_basic() {
    let mut state = SystemState::new();
    state.cpu.pc = 100;

    call(&mut state, 200, 1);

    assert_eq!(state.cpu.pc, 200);
    assert_eq!(state.cpu.registers[1], 104);
}

#[test]
fn test_call_multiple() {
    let mut state = SystemState::new();
    state.cpu.pc = 50;

    call(&mut state, 300, 2);

    assert_eq!(state.cpu.pc, 300);
    assert_eq!(state.cpu.registers[2], 54);
}
