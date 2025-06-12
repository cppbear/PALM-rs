// Answer 0

#[test]
fn test_slice_eq_equal_length_equal_elements() {
    let left = &[1, 2, 3];
    let right = &[1, 2, 3];
    let result = slice_eq(left, right, |a, b| a == b);
    assert_eq!(result, true);
}

#[test]
fn test_slice_eq_equal_length_different_elements() {
    let left = &[1, 2, 3];
    let right = &[1, 2, 4];
    let result = slice_eq(left, right, |a, b| a == b);
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
fn test_slice_eq_different_length() {
    let left = &[1, 2];
    let right = &[1, 2, 3];
    let result = slice_eq(left, right, |a, b| a == b);
    assert_eq!(result, false);
}

