// Answer 0

#[test]
fn test_capacity_to_buckets_small_cap() {
    assert_eq!(capacity_to_buckets(1), Some(4));
    assert_eq!(capacity_to_buckets(2), Some(4));
    assert_eq!(capacity_to_buckets(3), Some(4));
    assert_eq!(capacity_to_buckets(4), Some(8));
    assert_eq!(capacity_to_buckets(5), Some(8));
    assert_eq!(capacity_to_buckets(6), Some(8));
    assert_eq!(capacity_to_buckets(7), Some(8));
}

#[test]
fn test_capacity_to_buckets_large_cap() {
    assert_eq!(capacity_to_buckets(8), Some(16));
    assert_eq!(capacity_to_buckets(15), Some(32));
    assert_eq!(capacity_to_buckets(16), Some(32));
    assert_eq!(capacity_to_buckets(31), Some(64));
    assert_eq!(capacity_to_buckets(32), Some(64));
}

#[test]
fn test_capacity_to_buckets_boundary() {
    assert_eq!(capacity_to_buckets(0), None);
    assert_eq!(capacity_to_buckets(usize::MAX), None);
}

