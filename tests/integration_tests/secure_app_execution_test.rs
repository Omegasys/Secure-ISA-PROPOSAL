#[derive(Clone)]
struct Sandbox {
    mem_base: u64,
    mem_limit: u64,
}

fn execute_app(sb: &Sandbox, addr: u64) -> bool {
    addr >= sb.mem_base && addr < sb.mem_limit
}

#[test]
fn test_secure_app_execution() {
    let sandbox = Sandbox {
        mem_base: 0,
        mem_limit: 100,
    };

    // valid execution
    assert!(execute_app(&sandbox, 50));

    // attempted escape
    assert!(!execute_app(&sandbox, 150));
}
