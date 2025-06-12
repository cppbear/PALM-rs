// Answer 0

#[test]
fn test_peek_with_some_byte_zero() {
    let buffer: Vec<u8> = vec![b'a', b'b', b'c'];
    let bytes = buffer.iter().copied();
    let iter = LineColIterator { iter: bytes, line: 1, col: 0, start_of_line: 0 };
    let mut io_read = IoRead { iter, ch: Some(0), raw_buffer: None };
    let result = io_read.peek();
}

#[test]
fn test_peek_with_some_byte_mid() {
    let buffer: Vec<u8> = vec![b'a', b'b', b'c'];
    let bytes = buffer.iter().copied();
    let iter = LineColIterator { iter: bytes, line: 1, col: 0, start_of_line: 0 };
    let mut io_read = IoRead { iter, ch: Some(127), raw_buffer: None };
    let result = io_read.peek();
}

#[test]
fn test_peek_with_some_byte_max() {
    let buffer: Vec<u8> = vec![b'a', b'b', b'c'];
    let bytes = buffer.iter().copied();
    let iter = LineColIterator { iter: bytes, line: 1, col: 0, start_of_line: 0 };
    let mut io_read = IoRead { iter, ch: Some(255), raw_buffer: None };
    let result = io_read.peek();
}

