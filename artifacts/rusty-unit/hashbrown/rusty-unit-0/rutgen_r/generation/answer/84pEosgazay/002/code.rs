// Answer 0

#[test]
fn test_bucket_mask_to_capacity_case_equal_to_8() {
    let bucket_mask = 8;
    let expected_capacity = ((bucket_mask + 1) / 8) * 7;
    let actual_capacity = bucket_mask_to_capacity(bucket_mask);
    assert_eq!(actual_capacity, expected_capacity);
}

#[test]
fn test_bucket_mask_to_capacity_case_just_above_8() {
    let bucket_mask = 9;
    let expected_capacity = ((bucket_mask + 1) / 8) * 7;
    let actual_capacity = bucket_mask_to_capacity(bucket_mask);
    assert_eq!(actual_capacity, expected_capacity);
}

#[test]
fn test_bucket_mask_to_capacity_case_15() {
    let bucket_mask = 15;
    let expected_capacity = ((bucket_mask + 1) / 8) * 7;
    let actual_capacity = bucket_mask_to_capacity(bucket_mask);
    assert_eq!(actual_capacity, expected_capacity);
}

#[test]
fn test_bucket_mask_to_capacity_case_16() {
    let bucket_mask = 16;
    let expected_capacity = ((bucket_mask + 1) / 8) * 7;
    let actual_capacity = bucket_mask_to_capacity(bucket_mask);
    assert_eq!(actual_capacity, expected_capacity);
}

