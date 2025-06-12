// Answer 0

#[test]
fn test_slice_eq_different_lengths() {
    let left = [1, 2, 3];
    let right = [4, 5];
    let eq = |a, b| a == b;
    slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_empty_left_non_empty_right() {
    let left: &[i32] = &[];
    let right = [1, 2];
    let eq = |a, b| a == b;
    slice_eq(left, &right, eq);
}

#[test]
fn test_slice_eq_non_empty_left_empty_right() {
    let left = [1, 2];
    let right: &[i32] = &[];
    let eq = |a, b| a == b;
    slice_eq(&left, right, eq);
}

#[test]
fn test_slice_eq_different_lengths_strings() {
    let left = ["a", "b", "c"];
    let right = ["d", "e"];
    let eq = |a: &str, b: &str| a == b;
    slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_different_types() {
    let left = [1, 2, 3];
    let right = ["1", "2"];
    let eq = |a: &i32, b: &&str| a.to_string() == *b;
    slice_eq(&left, &right, eq);
}

