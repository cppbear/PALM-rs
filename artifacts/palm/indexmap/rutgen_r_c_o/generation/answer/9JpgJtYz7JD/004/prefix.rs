// Answer 0

#[test]
fn test_slice_eq_equal_non_empty() {
    let left = vec![1, 2, 3];
    let right = vec![1, 2, 3];
    let result = slice_eq(&left, &right, |a, b| a == b);
}

#[test]
fn test_slice_eq_equal_non_empty_strings() {
    let left = vec!["apple", "banana", "cherry"];
    let right = vec!["apple", "banana", "cherry"];
    let result = slice_eq(&left, &right, |a: &str, b: &str| a == b);
}

#[test]
fn test_slice_eq_different_elements_non_empty() {
    let left = vec![1, 2, 3];
    let right = vec![1, 2, 4];
    let result = slice_eq(&left, &right, |a, b| a == b);
}

#[test]
fn test_slice_eq_empty_slices() {
    let left: Vec<i32> = vec![];
    let right: Vec<i32> = vec![];
    let result = slice_eq(&left, &right, |a, b| a == b);
}

#[test]
fn test_slice_eq_different_lengths() {
    let left = vec![1, 2, 3];
    let right = vec![1, 2];
    let result = slice_eq(&left, &right, |a, b| a == b);
}

#[test]
fn test_slice_eq_identical_references() {
    let left = vec![&1, &2, &3];
    let right = vec![&1, &2, &3];
    let result = slice_eq(&left, &right, |a, b| a == b);
}

