// Answer 0

#[test]
fn test_simplify_range_unbounded_start() {
    let len = 5;
    let range = ..; // Unbounded start
    let result = simplify_range(range, len);
    assert_eq!(result, 0..len);
}

#[test]
fn test_simplify_range_included_start_within_length() {
    let len = 5;
    let range = 2..; // Included start
    let result = simplify_range(range, len);
    assert_eq!(result, 2..len);
}

#[test]
#[should_panic(expected = "range start index 6 out of range for slice of length 5")]
fn test_simplify_range_included_start_outside_length() {
    let len = 5;
    let range = 6..; // Included start outside length
    let _ = simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_start_within_length() {
    let len = 5;
    let range = ..4; // Excluded end
    let result = simplify_range(range, len);
    assert_eq!(result, 0..4);
}

#[test]
#[should_panic(expected = "range end index 5 out of range for slice of length 5")]
fn test_simplify_range_included_end_at_length() {
    let len = 5;
    let range = ..=5; // Included end at length
    let _ = simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range start index 5 should be <= range end index 4")]
fn test_simplify_range_inverted_range() {
    let len = 5;
    let range = 5..4; // Start greater than end
    let _ = simplify_range(range, len);
}

