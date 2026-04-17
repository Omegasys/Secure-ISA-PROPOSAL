use crate::core::state::SystemState;

fn output(state: &mut SystemState, device_id: u32, value: u8) -> Result<(), &'static str> {
    match state.io_devices.get_mut(&device_id) {
        Some(dev) => {
            dev.write(value);
            Ok(())
        }
        None => Err("DEVICE_NOT_FOUND"),
    }
}

#[test]
fn test_output_basic() {
    let mut state = SystemState::new();

    let device = crate::core::io::MockDevice::new(vec![]);
    state.io_devices.insert(1, Box::new(device));

    let result = output(&mut state, 1, 55);

    assert!(result.is_ok());
}

#[test]
fn test_output_invalid_device() {
    let mut state = SystemState::new();

    let result = output(&mut state, 999, 10);

    assert!(result.is_err());
}
