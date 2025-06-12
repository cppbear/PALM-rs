// Answer 0

#[test]
fn test_slice_eq_equal() {
    let left = &[1, 2, 3];
    let right = &[1, 2, 3];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(result);
}

#[test]
fn test_slice_eq_unequal() {
    let left = &[1, 2, 3];
    let right = &[1, 2, 4];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(!result);
}

#[test]
fn test_slice_eq_different_lengths() {
    let left = &[1, 2, 3];
    let right = &[1, 2];
    let result = slice_eq(left, right, |a, b| a == b);
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
fn test_slice_eq_type_differences() {
    let left = &[1.0, 2.0, 3.0];
    let right = &[1, 2, 3];
    let result = slice_eq(left, right, |a, b| (*a as i32) == *b);
    assert!(result);
}

