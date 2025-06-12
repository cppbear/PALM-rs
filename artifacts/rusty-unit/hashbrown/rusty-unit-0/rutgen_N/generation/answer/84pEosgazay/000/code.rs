// Answer 0

#[test]
fn test_bucket_mask_to_capacity_small_buckets() {
    assert_eq!(bucket_mask_to_capacity(0), 0);
    assert_eq!(bucket_mask_to_capacity(1), 1);
    assert_eq!(bucket_mask_to_capacity(2), 2);
    assert_eq!(bucket_mask_to_capacity(3), 3);
    assert_eq!(bucket_mask_to_capacity(4), 4);
    assert_eq!(bucket_mask_to_capacity(5), 5);
    assert_eq!(bucket_mask_to_capacity(6), 6);
    assert_eq!(bucket_mask_to_capacity(7), 7);
    assert_eq!(bucket_mask_to_capacity(8), 8);
}

#[test]
fn test_bucket_mask_to_capacity_large_buckets() {
    assert_eq!(bucket_mask_to_capacity(9), 7);
    assert_eq!(bucket_mask_to_capacity(15), 13);
    assert_eq!(bucket_mask_to_capacity(16), 14);
    assert_eq!(bucket_mask_to_capacity(23), 20);
    assert_eq!(bucket_mask_to_capacity(31), 27);
}

#[test]
fn test_bucket_mask_to_capacity_edge_cases() {
    assert_eq!(bucket_mask_to_capacity(8), 8);
    assert_eq!(bucket_mask_to_capacity(16), 14);
    assert_eq!(bucket_mask_to_capacity(32), 28);
    assert_eq!(bucket_mask_to_capacity(64), 56);
}

