// Answer 0

#[test]
fn test_simplify_range_unbounded() {
    use std::ops::{Bound, Range, RangeBounds};

    let result = simplify_range(.., 10);
    assert_eq!(result, 0..10);
}

#[test]
fn test_simplify_range_inclusive_start() {
    use std::ops::{Bound, RangeBounds};

    let result = simplify_range(1..4, 10);
    assert_eq!(result, 1..4);
}

#[test]
fn test_simplify_range_exclusive_start() {
    use std::ops::{Bound, RangeBounds};

    let result = simplify_range(1..=5, 10);
    assert_eq!(result, 2..6);
}

#[test]
fn test_simplify_range_inclusive_end() {
    use std::ops::{Bound, RangeBounds};

    let result = simplify_range(0..=5, 10);
    assert_eq!(result, 0..6);
}

#[test]
fn test_simplify_range_exclusive_end() {
    use std::ops::{Bound, RangeBounds};

    let result = simplify_range(0..5, 10);
    assert_eq!(result, 0..5);
}

#[test]
#[should_panic(expected = "range start index 10 out of range for slice of length 10")]
fn test_simplify_range_start_out_of_range() {
    use std::ops::{Bound, RangeBounds};

    let _ = simplify_range(10..5, 10);
}

#[test]
#[should_panic(expected = "range end index 10 out of range for slice of length 10")]
fn test_simplify_range_end_out_of_range() {
    use std::ops::{Bound, RangeBounds};

    let _ = simplify_range(0..10, 10);
}

#[test]
#[should_panic(expected = "range start index 5 should be <= range end index 5")]
fn test_simplify_range_start_greater_than_end() {
    use std::ops::{Bound, RangeBounds};

    let _ = simplify_range(5..5, 10);
}

