// Answer 0

#[test]
fn test_peek_position_min() {
    let iter = LineColIterator {
        iter: std::iter::empty(),
        line: 1,
        col: 1,
        start_of_line: 0,
    };
    let io_read = IoRead {
        iter,
        ch: None,
        raw_buffer: None,
    };
    let _ = io_read.peek_position();
}

#[test]
fn test_peek_position_edge_case() {
    let iter = LineColIterator {
        iter: std::iter::once(0_u8),
        line: 1000,
        col: 100,
        start_of_line: 9999,
    };
    let io_read = IoRead {
        iter,
        ch: None,
        raw_buffer: None,
    };
    let _ = io_read.peek_position();
}

#[test]
fn test_peek_position_max() {
    let iter = LineColIterator {
        iter: std::iter::repeat(0_u8).take(10000),
        line: 1000,
        col: 100,
        start_of_line: 9999,
    };
    let io_read = IoRead {
        iter,
        ch: None,
        raw_buffer: None,
    };
    let _ = io_read.peek_position();
}

#[test]
fn test_peek_position_random() {
    let iter = LineColIterator {
        iter: std::iter::once(0_u8),
        line: 500,
        col: 50,
        start_of_line: 4999,
    };
    let io_read = IoRead {
        iter,
        ch: None,
        raw_buffer: None,
    };
    let _ = io_read.peek_position();
}

