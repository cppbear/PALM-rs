// Answer 0

#[test]
fn test_len_non_empty_slice() {
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    assert_eq!(slice.len(), 5);
}

#[test]
fn test_len_empty_slice() {
    let slice: &[i32] = &[];
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_len_single_element_slice() {
    let slice: &[i32] = &[42];
    assert_eq!(slice.len(), 1);
}

