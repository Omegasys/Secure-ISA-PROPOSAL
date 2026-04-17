use crate::core::cpu::CPU;

#[test]
fn test_move_register() {
    let mut cpu = CPU::new();

    cpu.registers[1] = 123;
    cpu.registers[2] = cpu.registers[1];

    assert_eq!(cpu.registers[2], 123);
}
