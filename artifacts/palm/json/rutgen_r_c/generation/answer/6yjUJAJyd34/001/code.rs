// Answer 0

#[test]
fn test_position_first_byte() {
    let slice = b"Hello, world!";
    let mut reader = SliceRead::new(slice);
    reader.index = 0; // position of the first byte
    let position = reader.position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 0);
}

#[test]
fn test_position_middle_of_line() {
    let slice = b"Hello, world!";
    let mut reader = SliceRead::new(slice);
    reader.index = 7; // position of 'w'
    let position = reader.position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 7);
}

#[test]
fn test_position_newline() {
    let slice = b"Hello,\nworld!";
    let mut reader = SliceRead::new(slice);
    reader.index = 6; // position of '\n'
    let position = reader.position();
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 0);
}

#[test]
fn test_position_end_of_line() {
    let slice = b"Hello,\nworld!";
    let mut reader = SliceRead::new(slice);
    reader.index = 12; // position after 'd'
    let position = reader.position();
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 5);
}

#[test]
fn test_position_empty_slice() {
    let slice = b"";
    let mut reader = SliceRead::new(slice);
    reader.index = 0; // position of the first byte (not available)
    let position = reader.position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 0);
}

