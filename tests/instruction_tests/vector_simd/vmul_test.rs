fn vmul(a: &[u64], b: &[u64]) -> Vec<u64> {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()
}

#[test]
fn test_vmul_basic() {
    let a = vec![2, 3, 4];
    let b = vec![5, 6, 7];

    let result = vmul(&a, &b);

    assert_eq!(result, vec![10, 18, 28]);
}

#[test]
fn test_vmul_zero() {
    let a = vec![1, 2, 3];
    let b = vec![0, 0, 0];

    assert_eq!(vmul(&a, &b), vec![0, 0, 0]);
}
