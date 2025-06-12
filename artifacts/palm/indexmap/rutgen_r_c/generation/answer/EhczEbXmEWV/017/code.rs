// Answer 0

#[test]
fn test_simplify_range_unbounded_start() {
    let range = (..);
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 0..5);
}

#[test]
fn test_simplify_range_included_start() {
    let range = 1..3;
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 1..3);
}

#[test]
fn test_simplify_range_excluded_start() {
    let range = 1..=3;
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 2..4);
}

#[test]
#[should_panic(expected = "range start index 5 out of range for slice of length 5")]
fn test_simplify_range_included_start_out_of_bounds() {
    let range = 5..10;
    let len = 5;
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range start index 6 out of range for slice of length 5")]
fn test_simplify_range_excluded_start_out_of_bounds() {
    let range = 6..5;
    let len = 5;
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range start index 5 should be <= range end index 5")]
fn test_simplify_range_start_greater_than_end() {
    let range = 5..=5;
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_included_end_valid() {
    let range = ..=3;
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 0..4);
}

#[test]
#[should_panic(expected = "range end index 5 out of range for slice of length 5")]
fn test_simplify_range_included_end_out_of_bounds() {
    let range = ..=5;
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_excluded_end_valid() {
    let range = ..4;
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 0..4);
}

#[test]
#[should_panic(expected = "range end index 6 out of range for slice of length 5")]
fn test_simplify_range_excluded_end_out_of_bounds() {
    let range = ..6;
    let len = 5;
    simplify_range(range, len);
}

