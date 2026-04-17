use crate::core::state::SystemState;
use std::collections::HashSet;

#[derive(Clone)]
struct Sandbox {
    allowed_devices: HashSet<u32>,
}

fn can_access_device(sb: &Sandbox, device_id: u32) -> bool {
    sb.allowed_devices.contains(&device_id)
}

#[test]
fn test_io_allowed_device() {
    let mut allowed = HashSet::new();
    allowed.insert(1);

    let sb = Sandbox { allowed_devices: allowed };

    assert!(can_access_device(&sb, 1));
}

#[test]
fn test_io_denied_device() {
    let sb = Sandbox {
        allowed_devices: HashSet::new(),
    };

    assert!(!can_access_device(&sb, 2));
}
