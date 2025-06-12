// Answer 0

#[test]
fn test_helper_with_equal_bounds() {
    let bounds = (5, Some(5));
    let result = helper(bounds);
    assert_eq!(result, Some(5));
}

#[test]
fn test_helper_with_zero_bounds() {
    let bounds = (0, Some(0));
    let result = helper(bounds);
    assert_eq!(result, Some(0));
}

#[test]
fn test_helper_with_large_equal_bounds() {
    let bounds = (1_000_000, Some(1_000_000));
    let result = helper(bounds);
    assert_eq!(result, Some(1_000_000));
}

#[test]
fn test_helper_with_null_upper() {
    let bounds = (10, None);
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_with_lower_greater_than_upper() {
    let bounds = (10, Some(5));
    let result = helper(bounds);
    assert_eq!(result, None);
}

