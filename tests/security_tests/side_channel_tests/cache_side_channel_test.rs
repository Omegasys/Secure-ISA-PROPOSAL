use std::collections::HashMap;

// Simulated cache: tracks accessed indices
#[derive(Default)]
struct CacheSim {
    accessed: Vec<usize>,
}

fn access_array(cache: &mut CacheSim, data: &[u8], index: usize) -> u8 {
    cache.accessed.push(index);
    data[index]
}

// Secret-dependent access (BAD)
fn insecure_lookup(cache: &mut CacheSim, table: &[u8], secret: usize) -> u8 {
    access_array(cache, table, secret)
}

// Constant-time lookup (GOOD)
fn constant_time_lookup(cache: &mut CacheSim, table: &[u8], secret: usize) -> u8 {
    let mut result = 0;
    for i in 0..table.len() {
        let mask = ((i == secret) as u8).wrapping_neg();
        result |= table[i] & mask;
        cache.accessed.push(i); // uniform access
    }
    result
}

#[test]
fn test_cache_leak_detection() {
    let table = vec![10, 20, 30, 40];
    let mut cache = CacheSim::default();

    insecure_lookup(&mut cache, &table, 2);

    // Only one index accessed → leaks secret
    assert_eq!(cache.accessed, vec![2]);
}

#[test]
fn test_constant_time_cache_behavior() {
    let table = vec![10, 20, 30, 40];
    let mut cache = CacheSim::default();

    constant_time_lookup(&mut cache, &table, 2);

    // All indices accessed → no leak
    assert_eq!(cache.accessed, vec![0, 1, 2, 3]);
}
