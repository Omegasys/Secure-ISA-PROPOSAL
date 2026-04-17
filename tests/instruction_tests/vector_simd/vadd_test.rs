fn vadd(a: &[u64], b: &[u64]) -> Vec<u64> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

#[test]
fn test_vadd_basic() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    let result = vadd(&a, &b);

    assert_eq!(result, vec![5, 7, 9]);
}

#[test]
fn test_vadd_zero() {
    let a = vec![1, 2, 3];
    let b = vec![0, 0, 0];

    assert_eq!(vadd(&a, &b), a);
}
