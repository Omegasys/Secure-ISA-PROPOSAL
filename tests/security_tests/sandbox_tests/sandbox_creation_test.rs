use crate::core::state::SystemState;

#[derive(Clone)]
struct Sandbox {
    id: u32,
    mem_base: u64,
    mem_limit: u64,
}

fn create_sandbox(state: &mut SystemState, id: u32, base: u64, limit: u64) -> Option<Sandbox> {
    if base >= limit {
        return None;
    }

    let sb = Sandbox { id, mem_base: base, mem_limit: limit };
    state.sandboxes.insert(id, sb.clone());
    Some(sb)
}

#[test]
fn test_sandbox_creation_valid() {
    let mut state = SystemState::new();

    let sb = create_sandbox(&mut state, 1, 0, 100);

    assert!(sb.is_some());
    assert!(state.sandboxes.contains_key(&1));
}

#[test]
fn test_sandbox_creation_invalid_bounds() {
    let mut state = SystemState::new();

    let sb = create_sandbox(&mut state, 1, 100, 50);

    assert!(sb.is_none());
}
