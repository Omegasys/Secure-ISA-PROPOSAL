use crate::core::state::SystemState;

fn map_device(state: &mut SystemState, device_id: u32) -> Result<(), &'static str> {
    if state.io_devices.contains_key(&device_id) {
        return Err("ALREADY_MAPPED");
    }

    state.io_devices.insert(device_id, Box::new(crate::core::io::MockDevice::new(vec![])));
    Ok(())
}

#[test]
fn test_map_device_basic() {
    let mut state = SystemState::new();

    let result = map_device(&mut state, 1);

    assert!(result.is_ok());
    assert!(state.io_devices.contains_key(&1));
}

#[test]
fn test_map_device_duplicate() {
    let mut state = SystemState::new();

    map_device(&mut state, 1).unwrap();
    let result = map_device(&mut state, 1);

    assert!(result.is_err());
}
