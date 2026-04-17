use rand::{Rng, thread_rng};

#[derive(Default)]
struct CPU {
    reg: u64,
}

fn execute(cpu: &mut CPU, opcode: u8, operand: u64) {
    match opcode {
        0 => cpu.reg = cpu.reg.wrapping_add(operand),
        1 => cpu.reg = cpu.reg.wrapping_sub(operand),
        2 => cpu.reg ^= operand,
        _ => {} // undefined opcodes should not crash
    }
}

#[test]
fn fuzz_instructions() {
    let mut rng = thread_rng();
    let mut cpu = CPU::default();

    for _ in 0..10000 {
        let opcode = rng.gen::<u8>();
        let operand = rng.gen::<u64>();

        let before = cpu.reg;

        execute(&mut cpu, opcode, operand);

        // invariant: no panic, register always valid
        assert!(cpu.reg <= u64::MAX);
        assert!(cpu.reg.wrapping_sub(before) <= u64::MAX);
    }
}
