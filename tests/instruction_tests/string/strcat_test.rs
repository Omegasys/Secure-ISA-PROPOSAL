use crate::core::memory::Capability;

fn strcat(dst: &mut Vec<u8>, src: &[u8], cap: &Capability) -> Result<(), &'static str> {
    let new_len = dst.len() + src.len();

    if (dst.as_ptr() as u64) < cap.base || (dst.as_ptr() as u64) >= cap.limit {
        return Err("CAPABILITY_VIOLATION");
    }

    if new_len > cap.limit as usize - cap.base as usize {
        return Err("BUFFER_OVERFLOW");
    }

    dst.extend_from_slice(src);
    Ok(())
}

#[test]
fn test_strcat_basic() {
    let mut dst = b"hello".to_vec();
    let src = b" world";

    let cap = Capability {
        base: dst.as_ptr() as u64,
        limit: (dst.as_ptr() as u64) + 20,
        permissions: 0b0011,
    };

    let result = strcat(&mut dst, src, &cap);

    assert!(result.is_ok());
    assert_eq!(dst, b"hello world");
}

#[test]
fn test_strcat_overflow() {
    let mut dst = b"hello".to_vec();
    let src = b" world!!!";

    let cap = Capability {
        base: dst.as_ptr() as u64,
        limit: (dst.as_ptr() as u64) + 8,
        permissions: 0b0011,
    };

    let result = strcat(&mut dst, src, &cap);

    assert!(result.is_err());
}
