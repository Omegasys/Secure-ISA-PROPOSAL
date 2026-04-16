pub fn add(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}

pub fn sub(a: u64, b: u64) -> u64 {
    a.wrapping_sub(b)
}

pub fn mul(a: u64, b: u64) -> u64 {
    a.wrapping_mul(b)
}

pub fn div(a: u64, b: u64) -> Option<u64> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
