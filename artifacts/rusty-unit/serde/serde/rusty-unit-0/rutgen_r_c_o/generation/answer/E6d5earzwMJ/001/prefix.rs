// Answer 0

#[test]
fn test_helper_lower_zero_upper_none() {
    let bounds = (0, None);
    helper(bounds);
}

#[test]
fn test_helper_lower_one_upper_none() {
    let bounds = (1, None);
    helper(bounds);
}

#[test]
fn test_helper_lower_usize_max_upper_none() {
    let bounds = (usize::MAX, None);
    helper(bounds);
}

#[test]
fn test_helper_lower_zero_upper_one() {
    let bounds = (0, Some(1));
    helper(bounds);
}

#[test]
fn test_helper_lower_one_upper_zero() {
    let bounds = (1, Some(0));
    helper(bounds);
}

#[test]
fn test_helper_lower_two_upper_three() {
    let bounds = (2, Some(3));
    helper(bounds);
}

