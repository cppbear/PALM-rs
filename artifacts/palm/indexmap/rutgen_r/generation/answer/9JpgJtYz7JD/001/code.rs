// Answer 0

#[test]
fn test_slice_eq_diff_length() {
    let left = &[1, 2, 3];
    let right = &[1, 2];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(!result);
}

#[test]
fn test_slice_eq_different_length_empty() {
    let left: &[i32] = &[];
    let right = &[1, 2, 3];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(!result);
}

#[test]
fn test_slice_eq_different_length_large() {
    let left = &[1, 2, 3, 4, 5, 6];
    let right = &[1, 2, 3, 4, 5];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(!result);
}

