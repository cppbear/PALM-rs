// Answer 0

#[test]
fn test_helper_lower_not_equal_upper() {
    let bounds = (5, Some(10));
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_lower_not_equal_upper_zero() {
    let bounds = (0, Some(1));
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_lower_not_equal_upper_negative() {
    let bounds = (3, Some(4));
    let result = helper(bounds);
    assert_eq!(result, None);
}

#[test]
fn test_helper_lower_equals_upper_as_none() {
    let bounds = (3, None);
    let result = helper(bounds);
    assert_eq!(result, None);
}

