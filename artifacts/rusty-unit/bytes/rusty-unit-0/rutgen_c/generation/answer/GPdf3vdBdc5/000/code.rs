// Answer 0

#[test]
fn test_original_capacity_to_repr_min_capacity() {
    let capacity = 0;
    let result = original_capacity_to_repr(capacity);
    assert_eq!(result, 0);
}

#[test]
fn test_original_capacity_to_repr_below_min_capacity() {
    let capacity = 5;
    let result = original_capacity_to_repr(capacity);
    assert_eq!(result, 0);
}

#[test]
fn test_original_capacity_to_repr_at_min_capacity() {
    let capacity = 512; // 2^9
    let result = original_capacity_to_repr(capacity);
    assert_eq!(result, 0); // leading_zeros should give max width
}

#[test]
fn test_original_capacity_to_repr_above_min_capacity() {
    let capacity = 1024; // 2^10
    let result = original_capacity_to_repr(capacity);
    assert_eq!(result, 1); // width will be reduced by one
}

#[test]
fn test_original_capacity_to_repr_edge_cases() {
    let capacity = 32768; // 2^15
    let result = original_capacity_to_repr(capacity);
    assert_eq!(result, 2); // leading_zeros will affect width

    let capacity = 2097152; // 2^21
    let result = original_capacity_to_repr(capacity);
    assert_eq!(result, 7); // leading_zeros will affect width
}

