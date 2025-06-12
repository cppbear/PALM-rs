// Answer 0

#[test]
fn test_slice_eq_different_lengths() {
    let left = &[1, 2, 3];
    let right = &[1, 2];
    let are_equal = slice_eq(left, right, |a, b| a == b);
    assert_eq!(are_equal, false);
}

#[test]
fn test_slice_eq_zero_length_vs_non_zero_length() {
    let left: &[i32] = &[];
    let right = &[1];
    let are_equal = slice_eq(left, right, |a, b| a == b);
    assert_eq!(are_equal, false);
}

#[test]
fn test_slice_eq_non_zero_length_vs_zero_length() {
    let left = &[1];
    let right: &[i32] = &[];
    let are_equal = slice_eq(left, right, |a, b| a == b);
    assert_eq!(are_equal, false);
}

#[test]
fn test_slice_eq_large_length_difference() {
    let left = &[1, 2, 3, 4, 5];
    let right = &[1, 2, 3, 4];
    let are_equal = slice_eq(left, right, |a, b| a == b);
    assert_eq!(are_equal, false);
}

