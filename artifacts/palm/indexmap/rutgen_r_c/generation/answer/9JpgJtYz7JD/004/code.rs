// Answer 0

#[test]
fn test_slice_eq_equal_length_equal_elements() {
    let left = [1, 2, 3]; 
    let right = [1, 2, 3];
    let result = slice_eq(&left, &right, |a, b| a == b);
    assert!(result);
}

#[test]
fn test_slice_eq_equal_length_unequal_elements() {
    let left = [1, 2, 3];
    let right = [1, 2, 4];
    let result = slice_eq(&left, &right, |a, b| a == b);
    assert!(!result);
}

#[test]
fn test_slice_eq_different_length() {
    let left = [1, 2, 3];
    let right = [1, 2];
    let result = slice_eq(&left, &right, |a, b| a == b);
    assert!(!result);
}

#[test]
fn test_slice_eq_empty_slices() {
    let left: &[i32] = &[];
    let right: &[i32] = &[];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(result);
}

#[test]
fn test_slice_eq_empty_left_non_empty_right() {
    let left: &[i32] = &[];
    let right = [1];
    let result = slice_eq(left, right.as_slice(), |a, b| a == b);
    assert!(!result);
}

#[test]
fn test_slice_eq_non_empty_left_empty_right() {
    let left = [1];
    let right: &[i32] = &[];
    let result = slice_eq(left.as_slice(), right, |a, b| a == b);
    assert!(!result);
}

