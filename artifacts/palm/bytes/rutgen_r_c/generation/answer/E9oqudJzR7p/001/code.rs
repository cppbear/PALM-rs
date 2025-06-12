// Answer 0

#[test]
fn test_original_capacity_from_repr_zero() {
    assert_eq!(original_capacity_from_repr(0), 0);
}

#[test]
fn test_original_capacity_from_repr_one() {
    let result = original_capacity_from_repr(1);
    assert_eq!(result, 2048); // 1 << (1 + (10 - 1))
}

#[test]
fn test_original_capacity_from_repr_two() {
    let result = original_capacity_from_repr(2);
    assert_eq!(result, 4096); // 1 << (2 + (10 - 1))
}

#[test]
fn test_original_capacity_from_repr_max() {
    let repr = MAX_ORIGINAL_CAPACITY_WIDTH; // testing the maximum representable value
    let result = original_capacity_from_repr(repr);
    assert_eq!(result, 32768); // 1 << (17 + (10 - 1))
}

