// Answer 0

#[test]
fn test_next_with_some_character() {
    let mut io_reader = IoRead {
        iter: LineColIterator {
            iter: vec![Ok(65), Ok(66), Ok(67)].into_iter(),
            line: 1,
            col: 1,
            start_of_line: 0,
        },
        ch: Some(42),
        raw_buffer: None,
    };

    let result = io_reader.next();
}

#[test]
fn test_next_with_none_character() {
    let mut io_reader = IoRead {
        iter: LineColIterator {
            iter: vec![Ok(65), Ok(66), Ok(67)].into_iter(),
            line: 1,
            col: 1,
            start_of_line: 0,
        },
        ch: None,
        raw_buffer: None,
    };

    let result = io_reader.next();
}

#[test]
fn test_next_with_io_error() {
    let mut io_reader: IoRead<impl io::Read> = IoRead {
        iter: LineColIterator {
            iter: vec![Err(io::Error::new(io::ErrorKind::Other, "error"))].into_iter(),
            line: 1,
            col: 1,
            start_of_line: 0,
        },
        ch: Some(42),
        raw_buffer: None,
    };

    let result = io_reader.next();
}

