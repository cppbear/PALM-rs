// Answer 0

#[test]
fn test_bucket_mask_to_capacity_below_minimum() {
    let result = bucket_mask_to_capacity(0);
    assert_eq!(result, 0);
}

#[test]
fn test_bucket_mask_to_capacity_equal_to_minimum() {
    let result = bucket_mask_to_capacity(8);
    assert_eq!(result, 7);
}

#[test]
fn test_bucket_mask_to_capacity_above_minimum() {
    let result = bucket_mask_to_capacity(9);
    assert_eq!(result, 7);
}

#[test]
fn test_bucket_mask_to_capacity_large_value() {
    let result = bucket_mask_to_capacity(15);
    assert_eq!(result, 13);
}

#[test]
fn test_bucket_mask_to_capacity_divisible_by_8() {
    let result = bucket_mask_to_capacity(16);
    assert_eq!(result, 14);
}

#[test]
fn test_bucket_mask_to_capacity_conditions() {
    let result = bucket_mask_to_capacity(31);
    assert_eq!(result, 27);
}

