// Answer 0

#[test]
fn test_slice_eq_different_elements() {
    let left = [1];
    let right = [2];
    let eq = |a: &i32, b: &i32| a == b;
    slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_multiple_elements() {
    let left = [1, 2, 3];
    let right = [3, 2, 1];
    let eq = |a: &i32, b: &i32| a == b;
    slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_identical_sizes_different_values() {
    let left = [5, 6, 7];
    let right = [6, 5, 8];
    let eq = |a: &i32, b: &i32| a == b;
    slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_single_pair_diff() {
    let left = [10];
    let right = [20];
    let eq = |a: &i32, b: &i32| a == b;
    slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_all_elements_diff() {
    let left = [0, 1, 2, 3];
    let right = [4, 5, 6, 7];
    let eq = |a: &i32, b: &i32| a == b;
    slice_eq(&left, &right, eq);
}

