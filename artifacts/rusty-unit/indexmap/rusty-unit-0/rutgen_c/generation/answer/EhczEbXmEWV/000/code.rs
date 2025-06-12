// Answer 0

#[test]
fn test_simplify_range_included_bounds_within_length() {
    let range = 1..3;
    let result = simplify_range(range, 5);
    assert_eq!(result, 1..3);
}

#[test]
fn test_simplify_range_excluded_start() {
    let range = 1..=3;
    let result = simplify_range(range, 5);
    assert_eq!(result, 1..4);
}

#[test]
fn test_simplify_range_unbounded_start() {
    let range = ..3;
    let result = simplify_range(range, 5);
    assert_eq!(result, 0..3);
}

#[test]
fn test_simplify_range_unbounded_end() {
    let range = 1..;
    let result = simplify_range(range, 5);
    assert_eq!(result, 1..5);
}

#[test]
fn test_simplify_range_start_out_of_bounds() {
    let range = 6..=3;
    let result = std::panic::catch_unwind(|| {
        simplify_range(range, 5)
    });
    assert!(result.is_err());
}

#[test]
fn test_simplify_range_end_out_of_bounds() {
    let range = 1..6;
    let result = std::panic::catch_unwind(|| {
        simplify_range(range, 5)
    });
    assert!(result.is_err());
}

#[test]
fn test_simplify_range_start_greater_than_end() {
    let range = 3..1;
    let result = std::panic::catch_unwind(|| {
        simplify_range(range, 5)
    });
    assert!(result.is_err());
}

#[test]
fn test_simplify_range_exact_length() {
    let range = 0..5;
    let result = simplify_range(range, 5);
    assert_eq!(result, 0..5);
}

