pub fn key_exchange(private: u64, public: u64) -> u64 {
    private.wrapping_mul(public) ^ 0xA5A5A5A5
}
