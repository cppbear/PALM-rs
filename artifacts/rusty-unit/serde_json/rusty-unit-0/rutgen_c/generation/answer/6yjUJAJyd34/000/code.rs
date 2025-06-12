// Answer 0

#[test]
fn test_position_initial() {
    let slice = b"Hello, world!";
    let mut reader = SliceRead::new(slice);
    let pos = reader.position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_position_after_index_increment() {
    let slice = b"Hello,\nworld!";
    let mut reader = SliceRead::new(slice);
    reader.index = 7; // Pointing at the first character of "world!"
    let pos = reader.position();
    assert_eq!(pos.line, 2);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_position_with_multiple_lines() {
    let slice = b"Line 1\nLine 2\nLine 3";
    let mut reader = SliceRead::new(slice);
    reader.index = 12; // Pointing at the first character of "Line 3"
    let pos = reader.position();
    assert_eq!(pos.line, 3);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_position_with_empty_slice() {
    let slice: &[u8] = b"";
    let mut reader = SliceRead::new(slice);
    let pos = reader.position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 0);
}

