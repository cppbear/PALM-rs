// Answer 0

#[test]
fn test_helper_lower_equal_upper() {
    let bounds = (5, Some(5));
    assert_eq!(helper(bounds), Some(5));
}

#[test]
fn test_helper_lower_less_upper() {
    let bounds = (5, Some(10));
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_lower_greater_upper() {
    let bounds = (10, Some(5));
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_lower_equal_upper_none() {
    let bounds = (5, None);
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_lower_greater_upper_none() {
    let bounds = (10, None);
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_lower_zero_upper_zero() {
    let bounds = (0, Some(0));
    assert_eq!(helper(bounds), Some(0));
}

#[test]
fn test_helper_lower_zero_upper_five() {
    let bounds = (0, Some(5));
    assert_eq!(helper(bounds), None);
}

