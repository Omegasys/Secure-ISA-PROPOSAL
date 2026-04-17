use crate::core::state::SystemState;
use crate::core::memory::Capability;
use crate::execution::memory_ops::store;

#[test]
fn test_store_valid() {
    let mut state = SystemState::new();

    let cap = Capability {
        base: 0,
        limit: 10,
        permissions: 0b0010, // WRITE
    };

    let result = store(&mut state, 5, 77, &cap);

    assert!(result.is_ok());
    assert_eq!(state.memory.data.get(&5), Some(&77));
}

#[test]
fn test_store_out_of_bounds() {
    let mut state = SystemState::new();

    let cap = Capability {
        base: 0,
        limit: 5,
        permissions: 0b0010,
    };

    let result = store(&mut state, 6, 88, &cap);

    assert!(result.is_err());
}
