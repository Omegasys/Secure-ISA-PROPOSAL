fn vsub(a: &[u64], b: &[u64]) -> Vec<u64> {
    a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()
}

#[test]
fn test_vsub_basic() {
    let a = vec![5, 7, 9];
    let b = vec![1, 2, 3];

    let result = vsub(&a, &b);

    assert_eq!(result, vec![4, 5, 6]);
}

#[test]
fn test_vsub_underflow() {
    let a = vec![0];
    let b = vec![1];

    let result = vsub(&a, &b);

    assert_eq!(result[0], u64::MAX);
}
