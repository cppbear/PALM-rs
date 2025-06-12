// Answer 0

#[test]
fn test_simplify_range_unbounded_start() {
    let len = 5;
    let range = (..); // Unbounded start
    let result = simplify_range(range, len);
    assert_eq!(result, 0..5);
}

#[test]
fn test_simplify_range_included_start() {
    let len = 5;
    let range = (1..); // Included start
    let result = simplify_range(range, len);
    assert_eq!(result, 1..5);
}

#[test]
fn test_simplify_range_excluded_start() {
    let len = 5;
    let range = (1..4); // Excluded start and end
    let result = simplify_range(range, len);
    assert_eq!(result, 2..4);
}

#[test]
fn test_simplify_range_included_end() {
    let len = 5;
    let range = (1..=3); // Included start and end
    let result = simplify_range(range, len);
    assert_eq!(result, 1..4);
}

#[test]
fn test_simplify_range_excluded_end() {
    let len = 5;
    let range = (1..4); // Excluded end
    let result = simplify_range(range, len);
    assert_eq!(result, 1..4);
}

#[test]
#[should_panic(expected = "range start index 5 out of range for slice of length 5")]
fn test_simplify_range_panic_start_out_of_bound() {
    let len = 5;
    let range = (5..=len); // start out of bounds
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range end index 6 out of range for slice of length 5")]
fn test_simplify_range_panic_end_out_of_bound() {
    let len = 5;
    let range = (1..6); // end out of bounds
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range start index 5 should be <= range end index 4")]
fn test_simplify_range_start_greater_than_end() {
    let len = 5;
    let range = (5..4); // start greater than end
    simplify_range(range, len);
}

