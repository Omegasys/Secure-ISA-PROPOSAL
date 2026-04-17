#[derive(Clone)]
struct AuditEntry {
    data: String,
    checksum: u64,
}

fn checksum(data: &str) -> u64 {
    data.bytes().map(|b| b as u64).sum()
}

fn create_entry(data: &str) -> AuditEntry {
    AuditEntry {
        data: data.to_string(),
        checksum: checksum(data),
    }
}

fn verify(entry: &AuditEntry) -> bool {
    checksum(&entry.data) == entry.checksum
}

#[test]
fn test_valid_log_entry() {
    let entry = create_entry("EXEC");

    assert!(verify(&entry));
}

#[test]
fn test_tampered_log_entry() {
    let mut entry = create_entry("EXEC");

    entry.data = "MODIFIED".to_string();

    assert!(!verify(&entry));
}
