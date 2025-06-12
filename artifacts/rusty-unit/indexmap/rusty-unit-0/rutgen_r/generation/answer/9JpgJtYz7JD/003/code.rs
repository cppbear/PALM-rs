// Answer 0

#[test]
fn test_slice_eq_different_lengths() {
    let left = [1, 2, 3];
    let right = [1, 2];

    let result = slice_eq(&left, &right, |&l, &r| l == r);
    assert_eq!(result, false);
}

#[test]
fn test_slice_eq_different_elements() {
    let left = [1, 2, 3];
    let right = [1, 100, 3];

    let result = slice_eq(&left, &right, |&l, &r| l == r);
    assert_eq!(result, false);
}

#[test]
fn test_slice_eq_equal_elements() {
    let left = [1, 2, 3];
    let right = [1, 2, 3];

    let result = slice_eq(&left, &right, |&l, &r| l == r);
    assert_eq!(result, true);
}

