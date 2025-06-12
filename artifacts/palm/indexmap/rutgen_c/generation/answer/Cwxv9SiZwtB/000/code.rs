// Answer 0

#[test]
fn test_try_simplify_range_unbounded_start_and_end() {
    let range = ..;
    let result = try_simplify_range(range, 5);
    assert_eq!(result, Some(0..5));
}

#[test]
fn test_try_simplify_range_included_start_excluded_end() {
    let range = 2..Excluded(5);
    let result = try_simplify_range(range, 5);
    assert_eq!(result, Some(2..5));
}

#[test]
fn test_try_simplify_range_excluded_start_included_end() {
    let range = Excluded(2)..=4;
    let result = try_simplify_range(range, 5);
    assert_eq!(result, Some(3..5));
}

#[test]
fn test_try_simplify_range_included_to_excluded() {
    let range = 1..Excluded(3);
    let result = try_simplify_range(range, 5);
    assert_eq!(result, Some(1..3));
}

#[test]
fn test_try_simplify_range_invalid_start() {
    let range = Included(6)..=8;
    let result = try_simplify_range(range, 5);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_invalid_end() {
    let range = 1..Included(6);
    let result = try_simplify_range(range, 5);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_start_exceeds_end() {
    let range = 5..=3;
    let result = try_simplify_range(range, 5);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_empty() {
    let range = 5..5;
    let result = try_simplify_range(range, 5);
    assert_eq!(result, None);
}

