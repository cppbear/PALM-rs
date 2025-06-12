// Answer 0

#[test]
fn test_helper_lower_equals_upper() {
    let bounds = (3, Some(3)); // lower equals upper, should return Some(3)
    assert_eq!(helper(bounds), Some(3));
}

#[test]
fn test_helper_lower_not_equal_upper() {
    let bounds = (2, Some(3)); // lower not equals upper, should return None
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_lower_greater_than_upper() {
    let bounds = (4, Some(3)); // lower greater than upper, should return None
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_upper_none() {
    let bounds = (3, None); // upper is None, should return None
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_lower_none() {
    let bounds = (0, Some(0)); // lower is 0, upper equals lower, should return Some(0)
    assert_eq!(helper(bounds), Some(0));
}

