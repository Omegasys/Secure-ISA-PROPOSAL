use crate::core::state::SystemState;

fn unmap_device(state: &mut SystemState, device_id: u32) -> Result<(), &'static str> {
    match state.io_devices.remove(&device_id) {
        Some(_) => Ok(()),
        None => Err("DEVICE_NOT_FOUND"),
    }
}

#[test]
fn test_unmap_device_basic() {
    let mut state = SystemState::new();

    state.io_devices.insert(1, Box::new(crate::core::io::MockDevice::new(vec![])));

    let result = unmap_device(&mut state, 1);

    assert!(result.is_ok());
    assert!(!state.io_devices.contains_key(&1));
}

#[test]
fn test_unmap_device_invalid() {
    let mut state = SystemState::new();

    let result = unmap_device(&mut state, 1);

    assert!(result.is_err());
}
