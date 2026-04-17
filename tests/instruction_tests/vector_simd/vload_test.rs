use crate::core::state::SystemState;
use crate::core::memory::Capability;

fn vload(state: &SystemState, base: u64, len: usize, cap: &Capability) -> Option<Vec<u8>> {
    let mut result = Vec::new();

    for i in 0..len {
        let addr = base + i as u64;

        if addr < cap.base || addr >= cap.limit {
            return None;
        }

        match state.memory.data.get(&addr) {
            Some(v) => result.push(*v),
            None => result.push(0),
        }
    }

    Some(result)
}

#[test]
fn test_vload_basic() {
    let mut state = SystemState::new();

    state.memory.data.insert(0, 1);
    state.memory.data.insert(1, 2);
    state.memory.data.insert(2, 3);

    let cap = Capability {
        base: 0,
        limit: 10,
        permissions: 0b0001,
    };

    let result = vload(&state, 0, 3, &cap);

    assert_eq!(result, Some(vec![1, 2, 3]));
}

#[test]
fn test_vload_out_of_bounds() {
    let state = SystemState::new();

    let cap = Capability {
        base: 0,
        limit: 2,
        permissions: 0b0001,
    };

    let result = vload(&state, 0, 3, &cap);

    assert_eq!(result, None);
}
