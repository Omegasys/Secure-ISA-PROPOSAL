#[derive(Default)]
struct System {
    boot_verified: bool,
    memory_initialized: bool,
    root_privilege: bool,
}

fn secure_boot(system: &mut System) {
    system.boot_verified = true;
    system.memory_initialized = true;
    system.root_privilege = true;
}

#[test]
fn test_full_system_boot() {
    let mut system = System::default();

    secure_boot(&mut system);

    assert!(system.boot_verified);
    assert!(system.memory_initialized);
    assert!(system.root_privilege);
}
