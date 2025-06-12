// Answer 0

#[test]
fn test_slice_eq_equal_length_same_values() {
    let left = &[1, 2, 3];
    let right = &[1, 2, 3];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(result);
}

#[test]
fn test_slice_eq_equal_length_different_values() {
    let left = &[1, 2, 3];
    let right = &[1, 2, 4];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(!result);
}

#[test]
fn test_slice_eq_different_length() {
    let left = &[1, 2, 3];
    let right = &[1, 2];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(!result);
}

#[test]
fn test_slice_eq_equal_length_custom_struct() {
    #[derive(PartialEq)]
    struct Custom(i32);

    let left = &[Custom(1), Custom(2)];
    let right = &[Custom(1), Custom(2)];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(result);
}

#[test]
fn test_slice_eq_equal_length_partial_struct_different_value() {
    #[derive(PartialEq)]
    struct Custom(i32);

    let left = &[Custom(1), Custom(2)];
    let right = &[Custom(1), Custom(3)];
    let result = slice_eq(left, right, |a, b| a == b);
    assert!(!result);
}

