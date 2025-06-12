// Answer 0

#[test]
fn test_capacity_to_buckets_exact_boundary() {
    let cap: usize = 8;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(16));
}

#[test]
fn test_capacity_to_buckets_large_value() {
    let cap: usize = 10;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(16));
}

#[test]
fn test_capacity_to_buckets_edge_case() {
    let cap: usize = 14;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(32));
}

#[test]
fn test_capacity_to_buckets_high_value() {
    let cap: usize = 100;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(128));
}

