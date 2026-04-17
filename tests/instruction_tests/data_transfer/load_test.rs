use crate::core::state::SystemState;
use crate::core::memory::Capability;
use crate::execution::memory_ops::load;

#[test]
fn test_load_with_valid_capability() {
    let mut state = SystemState::new();

    let cap = Capability {
        base: 0,
        limit: 10,
        permissions: 0b0001, // READ
    };

    state.memory.data.insert(5, 42);

    let value = load(&mut state, 5, &cap);

    assert_eq!(value, Some(42));
}

#[test]
fn test_load_out_of_bounds() {
    let mut state = SystemState::new();

    let cap = Capability {
        base: 0,
        limit: 5,
        permissions: 0b0001,
    };

    state.memory.data.insert(6, 99);

    let value = load(&mut state, 6, &cap);

    assert_eq!(value, None);
}
