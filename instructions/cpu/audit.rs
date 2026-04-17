pub struct AuditLog {
    pub logs: Vec<String>,
}

impl AuditLog {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn record(&mut self, event: String) {
        self.logs.push(event);
    }

    pub fn dump(&self) {
        for log in &self.logs {
            println!("{}", log);
        }
    }
}