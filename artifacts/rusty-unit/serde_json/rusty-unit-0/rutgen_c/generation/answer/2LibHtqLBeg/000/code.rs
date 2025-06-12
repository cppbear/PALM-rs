// Answer 0

#[test]
fn test_peek_position_when_index_is_zero() {
    let slice = b"Hello, world!";
    let mut reader = SliceRead::new(slice);
    let position = reader.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 0);
}

#[test]
fn test_peek_position_when_index_is_within_bounds() {
    let slice = b"Hello, world!";
    let mut reader = SliceRead::new(slice);
    reader.index = 5; // point to the character 'o'
    let position = reader.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 5);
}

#[test]
fn test_peek_position_when_index_is_at_end() {
    let slice = b"Hello, world!";
    let mut reader = SliceRead::new(slice);
    reader.index = slice.len(); // point to the end of the slice
    let position = reader.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, slice.len());
}

#[test]
fn test_peek_position_after_new_line() {
    let slice = b"Hello,\nworld!";
    let mut reader = SliceRead::new(slice);
    reader.index = 7; // point to the character 'w'
    let position = reader.peek_position();
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 1);
}

