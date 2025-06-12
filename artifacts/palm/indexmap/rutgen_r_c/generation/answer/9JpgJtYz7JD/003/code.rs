// Answer 0

#[test]
fn test_slice_eq_different_lengths() {
    let left = [1, 2, 3];
    let right = [1, 2];
    let result = slice_eq(&left, &right, |a, b| a == b);
    assert_eq!(result, false);
}

#[test]
fn test_slice_eq_equal_elements() {
    let left = [1, 2, 3];
    let right = [1, 2, 3];
    let result = slice_eq(&left, &right, |a, b| a == b);
    assert_eq!(result, true);
}

#[test]
fn test_slice_eq_unequal_elements() {
    let left = [1, 2, 3];
    let right = [1, 2, 4];
    let result = slice_eq(&left, &right, |a, b| a == b);
    assert_eq!(result, false);
}

#[test]
fn test_slice_eq_empty_slices() {
    let left: &[i32] = &[];
    let right: &[i32] = &[];
    let result = slice_eq(left, right, |a, b| a == b);
    assert_eq!(result, true);
}

#[test]
fn test_slice_eq_one_element_equal() {
    let left = [5];
    let right = [5];
    let result = slice_eq(&left, &right, |a, b| a == b);
    assert_eq!(result, true);
}

#[test]
fn test_slice_eq_one_element_not_equal() {
    let left = [5];
    let right = [6];
    let result = slice_eq(&left, &right, |a, b| a == b);
    assert_eq!(result, false);
}

