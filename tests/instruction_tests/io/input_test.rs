use crate::core::state::SystemState;

fn input(state: &mut SystemState, device_id: u32) -> Option<u8> {
    state.io_devices.get(&device_id).and_then(|d| d.read())
}

#[test]
fn test_input_basic() {
    let mut state = SystemState::new();

    state.io_devices.insert(1, Box::new(crate::core::io::MockDevice::new(vec![42])));

    let value = input(&mut state, 1);

    assert_eq!(value, Some(42));
}

#[test]
fn test_input_no_device() {
    let mut state = SystemState::new();

    let value = input(&mut state, 999);

    assert_eq!(value, None);
}
