// Answer 0

#[test]
fn test_simplify_range_unbounded_start_included_end() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    let len = 10; 
    let range = 5..=8; 
    let result = simplify_range(range, len);
    assert_eq!(result, 5..9);
}

#[test]
fn test_simplify_range_unbounded_start_unbounded_end() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    let len = 15; 
    let range = ..; 
    let result = simplify_range(range, len);
    assert_eq!(result, 0..15);
}

#[test]
fn test_simplify_range_unbounded_start_excluded_end() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    let len = 20; 
    let range = ..9; 
    let result = simplify_range(range, len);
    assert_eq!(result, 0..9);
}

#[should_panic(expected = "range start index Unbounded should be <= range end index Included(10)")]
#[test]
fn test_simplify_range_start_greater_than_end() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    let len = 10; 
    let range = 11..=10; 
    simplify_range(range, len);
}

#[should_panic(expected = "range end index 10 out of range for slice of length 10")]
#[test]
fn test_simplify_range_included_end_out_of_range() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    let len = 10; 
    let range = 0..=10; 
    simplify_range(range, len);
}

#[should_panic(expected = "range start index 11 out of range for slice of length 10")]
#[test]
fn test_simplify_range_included_start_out_of_range() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    let len = 10; 
    let range = 11..=9; 
    simplify_range(range, len);
}

