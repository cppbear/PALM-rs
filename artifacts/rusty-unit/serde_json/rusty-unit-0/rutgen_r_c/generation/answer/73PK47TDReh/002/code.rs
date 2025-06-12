// Answer 0

#[test]
fn test_peek_at_empty_slice() {
    let slice: &[u8] = &[];
    let mut reader = SliceRead { slice, index: 0 };
    let result = reader.peek();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_peek_at_slice_with_data() {
    let slice: &[u8] = &[1, 2, 3];
    let mut reader = SliceRead { slice, index: 0 };
    let result = reader.peek();
    assert_eq!(result, Ok(Some(1)));
}

#[test]
fn test_peek_at_last_index() {
    let slice: &[u8] = &[1, 2, 3];
    let mut reader = SliceRead { slice, index: 3 };
    let result = reader.peek();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_peek_after_advanced_index() {
    let slice: &[u8] = &[1, 2, 3];
    let mut reader = SliceRead { slice, index: 4 }; // out of bounds
    let result = reader.peek();
    assert_eq!(result, Ok(None));
}

