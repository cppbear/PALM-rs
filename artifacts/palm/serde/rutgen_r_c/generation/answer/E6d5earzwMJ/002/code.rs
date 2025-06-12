// Answer 0

#[test]
fn test_helper_equal_bounds() {
    let bounds = (5, Some(5));
    let result = helper(bounds);
    assert_eq!(result, Some(5));
}

#[test]
fn test_helper_bounds_zero() {
    let bounds = (0, Some(0));
    let result = helper(bounds);
    assert_eq!(result, Some(0));
}

#[test]
fn test_helper_large_equal_bounds() {
    let bounds = (1000, Some(1000));
    let result = helper(bounds);
    assert_eq!(result, Some(1000));
}

#[test]
fn test_helper_upper_none() {
    let bounds = (5, None);
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_lower_greater_upper() {
    let bounds = (6, Some(5));
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_non_matching_bounds() {
    let bounds = (4, Some(6));
    let result = helper(bounds);
    assert_eq!(result, None);
}

