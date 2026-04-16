pub fn resolve_capability(id: &str) -> u64 {
    match id {
        "null" => 0,
        "kernel" => 0xFFFF0000,
        "user" => 0x0000FFFF,
        _ => 0x0,
    }
}
