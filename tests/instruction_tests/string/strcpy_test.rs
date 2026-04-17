use crate::core::memory::Capability;

fn strcpy(src: &[u8], dst: &mut [u8], cap: &Capability) -> Result<(), &'static str> {
    if src.len() > dst.len() {
        return Err("BUFFER_OVERFLOW");
    }

    if (dst.as_ptr() as u64) < cap.base || (dst.as_ptr() as u64) >= cap.limit {
        return Err("CAPABILITY_VIOLATION");
    }

    for i in 0..src.len() {
        dst[i] = src[i];
    }

    Ok(())
}

#[test]
fn test_strcpy_basic() {
    let src = b"hello";
    let mut dst = [0u8; 10];

    let cap = Capability {
        base: dst.as_ptr() as u64,
        limit: (dst.as_ptr() as u64) + 10,
        permissions: 0b0011,
    };

    let result = strcpy(src, &mut dst, &cap);

    assert!(result.is_ok());
    assert_eq!(&dst[..5], b"hello");
}

#[test]
fn test_strcpy_overflow() {
    let src = b"this is too long";
    let mut dst = [0u8; 5];

    let cap = Capability {
        base: dst.as_ptr() as u64,
        limit: (dst.as_ptr() as u64) + 5,
        permissions: 0b0011,
    };

    let result = strcpy(src, &mut dst, &cap);

    assert!(result.is_err());
}
