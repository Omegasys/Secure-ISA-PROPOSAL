#[derive(Default)]
struct AuditLog {
    entries: Vec<String>,
}

fn export_log(log: &AuditLog) -> String {
    log.entries.join("\n")
}

#[test]
fn test_export_log() {
    let mut log = AuditLog::default();

    log.entries.push("A".to_string());
    log.entries.push("B".to_string());

    let exported = export_log(&log);

    assert!(exported.contains("A"));
    assert!(exported.contains("B"));
}
