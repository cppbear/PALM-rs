// Answer 0

#[test]
fn test_capacity_to_buckets_minimum() {
    let cap = 4;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(4));
}

#[test]
fn test_capacity_to_buckets_exact_eight() {
    let cap = 8;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(16));
}

#[test]
fn test_capacity_to_buckets_large_value() {
    let cap = 100;
    let result = capacity_to_buckets(cap);
    assert_eq!(result, Some(128));
}

#[test]
fn test_capacity_to_buckets_overflow() {
    let cap = usize::MAX / 8 + 1;
    let result = capacity_to_buckets(cap);
    assert!(result.is_none());
}

