// Answer 0

#[test]
fn test_helper_return_none_lower_not_equal_upper() {
    // Case where lower is not equal to upper, should return None.
    let bounds1 = (5, Some(10));
    let result1 = helper(bounds1);
    assert_eq!(result1, None);

    let bounds2 = (3, Some(3));
    let result2 = helper(bounds2);
    assert_eq!(result2, None);

    let bounds3 = (4, Some(6));
    let result3 = helper(bounds3);
    assert_eq!(result3, None);

    let bounds4 = (0, Some(2));
    let result4 = helper(bounds4);
    assert_eq!(result4, None);
}

#[test]
fn test_helper_return_none_bounds_none() {
    // Case where upper is None, should return None.
    let bounds = (5, None);
    let result = helper(bounds);
    assert_eq!(result, None);
}

