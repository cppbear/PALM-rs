// Answer 0

#[test]
fn test_peek_position_empty_slice() {
    let read = SliceRead::new(&[]);
    let position = read.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 0);
}

#[test]
fn test_peek_position_single_byte() {
    let read = SliceRead::new(&[b'a']);
    let position = read.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 1);
}

#[test]
fn test_peek_position_multiple_bytes_no_newline() {
    let read = SliceRead::new(&[b'a', b'b', b'c']);
    let position = read.peek_position();
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 1);
}

#[test]
fn test_peek_position_multiple_bytes_with_newline() {
    let read = SliceRead::new(&[b'a', b'\n', b'b', b'c']);
    let position = read.peek_position();
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 1);
}

#[test]
fn test_peek_position_at_index_boundary() {
    let mut read = SliceRead::new(&[b'a', b'\n', b'b', b'c']);
    read.index = 3; // Pointing to the last byte 'c'
    let position = read.peek_position();
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 4); // It should cap at the end
}

#[test]
fn test_peek_position_after_end_of_slice() {
    let mut read = SliceRead::new(&[b'a', b'b', b'c']);
    read.index = 4; // Beyond the end of the slice
    let position = read.peek_position();
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 4); // It should cap at the end
}

