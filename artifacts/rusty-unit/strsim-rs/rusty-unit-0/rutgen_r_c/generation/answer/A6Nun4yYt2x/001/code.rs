// Answer 0

#[test]
fn test_flat_index_normal_case() {
    let width = 5;
    let i = 2;
    let j = 3;
    let expected = j * width + i;
    let result = flat_index(i, j, width);
    assert_eq!(result, expected);
}

#[test]
fn test_flat_index_first_element() {
    let width = 5;
    let i = 0;
    let j = 0;
    let expected = j * width + i;
    let result = flat_index(i, j, width);
    assert_eq!(result, expected);
}

#[test]
fn test_flat_index_last_element_in_row() {
    let width = 5;
    let i = 4;
    let j = 0;
    let expected = j * width + i;
    let result = flat_index(i, j, width);
    assert_eq!(result, expected);
}

#[test]
fn test_flat_index_multiple_rows() {
    let width = 10;
    let i = 3;
    let j = 2; // jumping to the third row
    let expected = j * width + i;
    let result = flat_index(i, j, width);
    assert_eq!(result, expected);
}

#[test]
fn test_flat_index_large_indices() {
    let width = 100;
    let i = 99; // last element in row
    let j = 99; // last row
    let expected = j * width + i;
    let result = flat_index(i, j, width);
    assert_eq!(result, expected);
}

