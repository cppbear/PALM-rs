// Answer 0

#[test]
fn test_split_at_mut_valid_index() {
    let mut slice: &mut [i32] = &mut [1, 2, 3, 4, 5];
    let (first, second) = slice.split_at_mut(3);
    assert_eq!(*first, [1, 2, 3]);
    assert_eq!(*second, [4, 5]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_split_at_mut_index_too_large() {
    let mut slice: &mut [i32] = &mut [1, 2, 3];
    let _ = slice.split_at_mut(4);
}

#[test]
fn test_split_at_mut_index_zero() {
    let mut slice: &mut [i32] = &mut [1, 2, 3, 4, 5];
    let (first, second) = slice.split_at_mut(0);
    assert_eq!(*first, []);
    assert_eq!(*second, [1, 2, 3, 4, 5]);
}

