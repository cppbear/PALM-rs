// Answer 0

#[test]
fn test_peek_with_valid_index() {
    let data: &[u8] = &[1, 2, 3, 4, 5];
    let mut slice_read = SliceRead {
        slice: data,
        index: 2,
    };

    let result = slice_read.peek().unwrap();
    assert_eq!(result, Some(3));
}

#[test]
fn test_peek_with_end_of_slice() {
    let data: &[u8] = &[1, 2, 3, 4, 5];
    let mut slice_read = SliceRead {
        slice: data,
        index: 5,
    };

    let result = slice_read.peek().unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_peek_with_empty_slice() {
    let data: &[u8] = &[];
    let mut slice_read = SliceRead {
        slice: data,
        index: 0,
    };

    let result = slice_read.peek().unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_peek_with_index_less_than_zero() {
    let data: &[u8] = &[1, 2, 3, 4, 5];
    let mut slice_read = SliceRead {
        slice: data,
        index: 0,
    };

    let result = slice_read.peek().unwrap();
    assert_eq!(result, Some(1));
}

