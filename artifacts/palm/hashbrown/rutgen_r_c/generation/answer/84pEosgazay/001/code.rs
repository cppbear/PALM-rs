// Answer 0

#[test]
fn test_bucket_mask_to_capacity_with_small_values() {
    assert_eq!(bucket_mask_to_capacity(0), 0);
    assert_eq!(bucket_mask_to_capacity(1), 1);
    assert_eq!(bucket_mask_to_capacity(2), 2);
    assert_eq!(bucket_mask_to_capacity(3), 3);
    assert_eq!(bucket_mask_to_capacity(4), 4);
    assert_eq!(bucket_mask_to_capacity(5), 5);
    assert_eq!(bucket_mask_to_capacity(6), 6);
    assert_eq!(bucket_mask_to_capacity(7), 7);
}

#[test]
fn test_bucket_mask_to_capacity_with_boundary_value() {
    assert_eq!(bucket_mask_to_capacity(8), 7);
}

