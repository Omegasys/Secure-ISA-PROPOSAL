#[derive(Default)]
struct Trace {
    instructions: Vec<String>,
}

fn trace_instruction(trace: &mut Trace, instr: &str) {
    trace.instructions.push(instr.to_string());
}

#[test]
fn test_instruction_trace() {
    let mut trace = Trace::default();

    trace_instruction(&mut trace, "ADD");
    trace_instruction(&mut trace, "SUB");

    assert_eq!(trace.instructions.len(), 2);
    assert_eq!(trace.instructions[0], "ADD");
}
