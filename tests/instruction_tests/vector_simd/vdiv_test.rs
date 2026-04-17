fn vdiv(a: &[u64], b: &[u64]) -> Vec<Option<u64>> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| if *y == 0 { None } else { Some(x / y) })
        .collect()
}

#[test]
fn test_vdiv_basic() {
    let a = vec![10, 20];
    let b = vec![2, 4];

    let result = vdiv(&a, &b);

    assert_eq!(result, vec![Some(5), Some(5)]);
}

#[test]
fn test_vdiv_by_zero() {
    let a = vec![10];
    let b = vec![0];

    let result = vdiv(&a, &b);

    assert_eq!(result, vec![None]);
}
