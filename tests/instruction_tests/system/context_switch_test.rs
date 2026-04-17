use crate::core::state::SystemState;

#[derive(Clone)]
struct Context {
    pc: u64,
    registers: [u64; 32],
}

fn context_switch(state: &mut SystemState, new_ctx: &Context) {
    state.cpu.pc = new_ctx.pc;
    state.cpu.registers = new_ctx.registers;
}

#[test]
fn test_context_switch_basic() {
    let mut state = SystemState::new();

    let ctx = Context {
        pc: 500,
        registers: [1; 32],
    };

    context_switch(&mut state, &ctx);

    assert_eq!(state.cpu.pc, 500);
    assert_eq!(state.cpu.registers[0], 1);
}
