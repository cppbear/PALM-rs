// Answer 0

#[test]
fn test_original_capacity_to_repr_zero() {
    let result = original_capacity_to_repr(0);
    assert_eq!(result, MAX_ORIGINAL_CAPACITY_WIDTH - MIN_ORIGINAL_CAPACITY_WIDTH);
}

#[test]
fn test_original_capacity_to_repr_min() {
    let result = original_capacity_to_repr(1);
    let expected = PTR_WIDTH - (1 >> MIN_ORIGINAL_CAPACITY_WIDTH).leading_zeros() as usize;
    assert_eq!(result, cmp::min(expected, MAX_ORIGINAL_CAPACITY_WIDTH - MIN_ORIGINAL_CAPACITY_WIDTH));
}

#[test]
fn test_original_capacity_to_repr_max() {
    let result = original_capacity_to_repr(usize::MAX);
    let expected = PTR_WIDTH - (usize::MAX >> MIN_ORIGINAL_CAPACITY_WIDTH).leading_zeros() as usize;
    assert_eq!(result, cmp::min(expected, MAX_ORIGINAL_CAPACITY_WIDTH - MIN_ORIGINAL_CAPACITY_WIDTH));
}

#[test]
fn test_original_capacity_to_repr_boundary() {
    let cap = (1 << (MAX_ORIGINAL_CAPACITY_WIDTH + MIN_ORIGINAL_CAPACITY_WIDTH - 1)) - 1;
    let result = original_capacity_to_repr(cap);
    let expected = PTR_WIDTH - (cap >> MIN_ORIGINAL_CAPACITY_WIDTH).leading_zeros() as usize;
    assert_eq!(result, cmp::min(expected, MAX_ORIGINAL_CAPACITY_WIDTH - MIN_ORIGINAL_CAPACITY_WIDTH));
}

