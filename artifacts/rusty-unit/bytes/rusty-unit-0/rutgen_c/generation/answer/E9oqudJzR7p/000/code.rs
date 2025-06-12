// Answer 0

#[test]
fn test_original_capacity_from_repr_zero() {
    let result = original_capacity_from_repr(0);
    assert_eq!(result, 0);
}

#[test]
fn test_original_capacity_from_repr_min() {
    let result = original_capacity_from_repr(MIN_ORIGINAL_CAPACITY_WIDTH - 1);
    assert_eq!(result, 1 << (MIN_ORIGINAL_CAPACITY_WIDTH - 1 + (MIN_ORIGINAL_CAPACITY_WIDTH - 1)));
}

#[test]
fn test_original_capacity_from_repr_lower_bound() {
    let result = original_capacity_from_repr(MIN_ORIGINAL_CAPACITY_WIDTH);
    assert_eq!(result, 1 << (MIN_ORIGINAL_CAPACITY_WIDTH + (MIN_ORIGINAL_CAPACITY_WIDTH - 1)));
}

#[test]
fn test_original_capacity_from_repr_upper_bound() {
    let result = original_capacity_from_repr(MAX_ORIGINAL_CAPACITY_WIDTH);
    assert_eq!(result, 1 << (MAX_ORIGINAL_CAPACITY_WIDTH + (MIN_ORIGINAL_CAPACITY_WIDTH - 1)));
}

