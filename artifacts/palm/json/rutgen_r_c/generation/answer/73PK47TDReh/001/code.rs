// Answer 0

#[test]
fn test_peek_with_valid_index() {
    let data: &[u8] = &[0, 1, 2, 3, 4, 5];
    let mut slice_reader = SliceRead {
        slice: data,
        index: 3, // Valid index within bounds
    };
    let result = slice_reader.peek().unwrap();
    assert_eq!(result, Some(3));
}

#[test]
fn test_peek_at_last_element() {
    let data: &[u8] = &[10, 20, 30];
    let mut slice_reader = SliceRead {
        slice: data,
        index: 2, // Index at the last element
    };
    let result = slice_reader.peek().unwrap();
    assert_eq!(result, Some(30));
}

#[test]
fn test_peek_with_index_at_end() {
    let data: &[u8] = &[5, 6, 7];
    let mut slice_reader = SliceRead {
        slice: data,
        index: 3, // Index out of bounds
    };
    let result = slice_reader.peek().unwrap();
    assert_eq!(result, None);
}

