// Answer 0

#[test]
fn test_io_read_new() {
    use std::io::{Cursor, Read};
    use serde_json::IoRead;

    // Given a simple byte array to represent input data
    let input_data = b"{\"key\":\"value\"}";
    let cursor = Cursor::new(input_data);

    // When creating a new IoRead instance
    let io_read: IoRead<Cursor<&[u8]>> = IoRead::new(cursor);

    // Assert that the instance was created successfully
    assert!(io_read.iter.line == 1);
    assert!(io_read.iter.col == 0);
}

#[test]
fn test_io_read_next() {
    use std::io::{Cursor, Read};
    use serde_json::{IoRead, Result};

    let input_data = b"hello";
    let cursor = Cursor::new(input_data);
    let mut io_read: IoRead<Cursor<&[u8]>> = IoRead::new(cursor);

    // When reading next byte
    let next_byte: Result<Option<u8>> = io_read.next();

    // Assert that the first byte read is 'h'
    assert_eq!(next_byte.ok(), Some(Some(b'h')));
}

#[test]
fn test_io_read_peek() {
    use std::io::{Cursor, Read};
    use serde_json::{IoRead, Result};

    let input_data = b"world";
    let cursor = Cursor::new(input_data);
    let mut io_read: IoRead<Cursor<&[u8]>> = IoRead::new(cursor);

    // When peeking the first byte
    let peeked_byte: Result<Option<u8>> = io_read.peek();

    // Assert that the byte peeked is 'w'
    assert_eq!(peeked_byte.ok(), Some(Some(b'w')));
}

#[test]
fn test_io_read_discard() {
    use std::io::{Cursor, Read};
    use serde_json::{IoRead};

    let input_data = b"test";
    let cursor = Cursor::new(input_data);
    let mut io_read: IoRead<Cursor<&[u8]>> = IoRead::new(cursor);

    // When discarding the input
    io_read.discard();

    // Assert that after discard the position should not have moved
    assert_eq!(io_read.position().line, 1);
    assert_eq!(io_read.position().col, 0);
}

