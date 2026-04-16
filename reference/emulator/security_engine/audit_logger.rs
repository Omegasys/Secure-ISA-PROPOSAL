use std::vec::Vec;

pub struct AuditEvent {
    pub event_type: String,
    pub cycle: u64,
    pub severity: u8,
}

pub struct AuditLog {
    pub events: Vec<AuditEvent>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn log(&mut self, event_type: &str, cycle: u64, severity: u8) {
        self.events.push(AuditEvent {
            event_type: event_type.to_string(),
            cycle,
            severity,
        });
    }

    pub fn export(&self) -> &Vec<AuditEvent> {
        &self.events
    }
}
