pub struct Clock {
    pub cycle: u64,
    pub deterministic_mode: bool,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            cycle: 0,
            deterministic_mode: true,
        }
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
    }
}
