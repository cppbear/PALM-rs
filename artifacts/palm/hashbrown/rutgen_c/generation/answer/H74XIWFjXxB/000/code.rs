// Answer 0

#[test]
fn test_capacity_to_buckets_with_zero() {
    let result = capacity_to_buckets(0);
    assert!(result.is_none());
}

#[test]
fn test_capacity_to_buckets_with_small_values() {
    assert_eq!(capacity_to_buckets(1), Some(4));
    assert_eq!(capacity_to_buckets(2), Some(4));
    assert_eq!(capacity_to_buckets(3), Some(4));
    assert_eq!(capacity_to_buckets(4), Some(8));
    assert_eq!(capacity_to_buckets(5), Some(8));
    assert_eq!(capacity_to_buckets(6), Some(8));
    assert_eq!(capacity_to_buckets(7), Some(8));
}

#[test]
fn test_capacity_to_buckets_with_large_values() {
    assert_eq!(capacity_to_buckets(8), Some(8));
    assert_eq!(capacity_to_buckets(16), Some(32));
    assert_eq!(capacity_to_buckets(15), Some(32));
    assert_eq!(capacity_to_buckets(32), Some(64));
    assert_eq!(capacity_to_buckets(63), Some(128));
    assert_eq!(capacity_to_buckets(64), Some(128));
}

#[test]
fn test_capacity_to_buckets_with_edge_case() {
    assert_eq!(capacity_to_buckets(7), Some(8));
    assert_eq!(capacity_to_buckets(8), Some(8));
}

