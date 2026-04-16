pub struct RatchetState {
    pub key: u64,
    pub step: u64,
}

impl RatchetState {
    pub fn new(key: u64) -> Self {
        Self { key, step: 0 }
    }

    pub fn advance(&mut self) {
        self.key ^= self.step.wrapping_add(0x9E3779B97F4A7C15);
        self.step += 1;
    }

    pub fn encrypt(&self, data: u64) -> u64 {
        data ^ self.key
    }
}
