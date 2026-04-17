fn strcmp(a: &[u8], b: &[u8]) -> i32 {
    let len = a.len().min(b.len());

    for i in 0..len {
        if a[i] != b[i] {
            return (a[i] as i32) - (b[i] as i32);
        }
    }

    (a.len() as i32) - (b.len() as i32)
}

#[test]
fn test_strcmp_equal() {
    assert_eq!(strcmp(b"abc", b"abc"), 0);
}

#[test]
fn test_strcmp_less() {
    assert!(strcmp(b"abc", b"abd") < 0);
}

#[test]
fn test_strcmp_greater() {
    assert!(strcmp(b"abe", b"abd") > 0);
}
