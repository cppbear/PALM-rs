// Answer 0

#[test]
fn test_simplify_range_unbounded_start() {
    let range = std::ops::RangeBounds::Unbounded; 
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 0..5);
}

#[test]
fn test_simplify_range_included_start() {
    let range = 3..5; 
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 3..5);
}

#[should_panic(expected = "range start index 5 out of range for slice of length 5")]
#[test]
fn test_simplify_range_included_start_out_of_bounds() {
    let range = 5..6; 
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_included_end() {
    let range = 0..4; 
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 0..4);
}

#[should_panic(expected = "range end index 5 out of range for slice of length 5")]
#[test]
fn test_simplify_range_included_end_out_of_bounds() {
    let range = 0..5; 
    let len = 5;
    simplify_range(range, len);
}

#[test]
fn test_simplify_range_included_start_and_end() {
    let range = 1..3; 
    let len = 5;
    let result = simplify_range(range, len);
    assert_eq!(result, 1..3);
}

#[should_panic(expected = "range start index 1 should be <= range end index 0")]
#[test]
fn test_simplify_range_reverse_order() {
    let range = 1..0; 
    let len = 5;
    simplify_range(range, len);
}

