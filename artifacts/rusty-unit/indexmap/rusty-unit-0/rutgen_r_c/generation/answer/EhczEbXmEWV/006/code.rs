// Answer 0

#[test]
fn test_simplify_range_unbounded_start() {
    let range = std::ops::RangeFull; // This represents an unbounded range
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 0..5); // Should simplify to the full range
}

#[test]
fn test_simplify_range_included_start() {
    let range = 1..5; // Included start and end range
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 1..5); // Should remain the same
}

#[test]
fn test_simplify_range_excluded_end() {
    let range = 1..=4; // Included start and excluded end
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 1..5); // Included start gives 1, Excluded end gives 5
}

#[test]
#[should_panic(expected = "range start index 5 out of range for slice of length 5")]
fn test_simplify_range_invalid_start_included() {
    let range = 5..10; // Start is out of range
    let len = 5;
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range end index 5 out of range for slice of length 5")]
fn test_simplify_range_invalid_end_included() {
    let range = 0..=5; // End is out of range
    let len = 5;
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range start index 5 should be <= range end index 4")]
fn test_simplify_range_start_greater_than_end() {
    let range = 5..=4; // Invalid range where start > end
    let len = 5;
    simplify_range(range, len);
}

