pub struct CryptoEngine {
    pub key: Vec<u8>,
}

impl CryptoEngine {
    pub fn new() -> Self {
        Self { key: vec![0xAA] }
    }

    pub fn encrypt(&self, data: u8) -> u8 {
        data ^ self.key[0] // placeholder
    }

    pub fn decrypt(&self, data: u8) -> u8 {
        data ^ self.key[0]
    }
}