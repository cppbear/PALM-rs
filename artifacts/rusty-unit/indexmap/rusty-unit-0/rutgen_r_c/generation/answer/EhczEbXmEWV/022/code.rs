// Answer 0

#[test]
fn test_simplify_range_included_start_within_bounds() {
    let result = simplify_range(1..4, 5);
    assert_eq!(result, 1..4);
}

#[test]
fn test_simplify_range_included_start_equal_to_length() {
    let result = simplify_range(5..8, 5);
    assert_eq!(result, 5..6);
}

#[test]
fn test_simplify_range_excluded_end_within_bounds() {
    let result = simplify_range(0..3, 4);
    assert_eq!(result, 0..3);
}

#[test]
fn test_simplify_range_excluded_end_equal_to_length() {
    let result = simplify_range(0..5, 5);
    assert_eq!(result, 0..5);
}

#[test]
#[should_panic(expected = "range end index 5 out of range for slice of length 5")]
fn test_simplify_range_excluded_end_out_of_bounds() {
    simplify_range(0..6, 5);
}

#[test]
#[should_panic(expected = "range start index 6 out of range for slice of length 5")]
fn test_simplify_range_included_start_out_of_bounds() {
    simplify_range(6..8, 5);
}

#[test]
#[should_panic(expected = "range start index 4 should be <= range end index 3")]
fn test_simplify_range_start_greater_than_end() {
    simplify_range(4..3, 5);
}

