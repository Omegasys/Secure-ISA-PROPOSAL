#[derive(Clone)]
struct Sandbox {
    allow_crypto: bool,
}

fn crypto_op(sb: &Sandbox, key: u8, data: &[u8]) -> Option<Vec<u8>> {
    if !sb.allow_crypto {
        return None;
    }

    Some(data.iter().map(|b| b ^ key).collect())
}

#[test]
fn test_crypto_pipeline_allowed() {
    let sb = Sandbox { allow_crypto: true };

    let result = crypto_op(&sb, 0xAA, b"data");

    assert!(result.is_some());
}

#[test]
fn test_crypto_pipeline_blocked() {
    let sb = Sandbox { allow_crypto: false };

    let result = crypto_op(&sb, 0xAA, b"data");

    assert!(result.is_none());
}
