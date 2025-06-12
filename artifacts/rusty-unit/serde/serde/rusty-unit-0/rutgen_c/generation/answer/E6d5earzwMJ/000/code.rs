// Answer 0

#[test]
fn test_helper_equal_bounds() {
    let bounds = (5, Some(5));
    assert_eq!(helper(bounds), Some(5));
}

#[test]
fn test_helper_non_equal_bounds() {
    let bounds = (5, Some(10));
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_lower_bound_greater() {
    let bounds = (10, Some(5));
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_none_upper_bound() {
    let bounds = (5, None);
    assert_eq!(helper(bounds), None);
}

#[test]
fn test_helper_zero_bounds() {
    let bounds = (0, Some(0));
    assert_eq!(helper(bounds), Some(0));
}

#[test]
fn test_helper_empty_bounds() {
    let bounds = (1, Some(0));
    assert_eq!(helper(bounds), None);
}

