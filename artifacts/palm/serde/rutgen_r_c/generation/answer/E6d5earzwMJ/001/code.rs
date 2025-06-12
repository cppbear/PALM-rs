// Answer 0

#[test]
fn test_helper_none_lower_not_equal_upper() {
    let bounds = (3, Some(5));
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_none_lower_greater_than_upper() {
    let bounds = (6, Some(5));
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_none_upper_none() {
    let bounds = (2, None);
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_none_lower_equal_zero_upper_non_zero() {
    let bounds = (0, Some(1));
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_none_lower_zero_upper_zero() {
    let bounds = (0, Some(0));
    let result = helper(bounds);
    assert_eq!(result, Some(0));
}

