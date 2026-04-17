#[derive(Default)]
struct AuditLog {
    entries: Vec<String>,
}

fn log_event(log: &mut AuditLog, event: &str) {
    log.entries.push(event.to_string());
}

#[test]
fn test_audit_log_creation() {
    let mut log = AuditLog::default();

    log_event(&mut log, "LOAD executed");

    assert_eq!(log.entries.len(), 1);
    assert_eq!(log.entries[0], "LOAD executed");
}
