// Answer 0

#[test]
fn test_set_failed_false_to_true() {
    let mut failed = false;
    let mut io_read = IoRead {
        iter: LineColIterator { iter: vec![].into_iter(), line: 0, col: 0, start_of_line: 0 },
        ch: None,
        raw_buffer: None,
    };
    io_read.set_failed(&mut failed);
}

#[test]
fn test_set_failed_true_to_true() {
    let mut failed = true;
    let mut io_read = IoRead {
        iter: LineColIterator { iter: vec![].into_iter(), line: 0, col: 0, start_of_line: 0 },
        ch: None,
        raw_buffer: None,
    };
    io_read.set_failed(&mut failed);
}

#[test]
fn test_set_failed_transition_true_false() {
    let mut failed = false;
    let mut io_read = IoRead {
        iter: LineColIterator { iter: vec![].into_iter(), line: 0, col: 0, start_of_line: 0 },
        ch: None,
        raw_buffer: None,
    };
    io_read.set_failed(&mut failed);
    io_read.set_failed(&mut failed);
}

