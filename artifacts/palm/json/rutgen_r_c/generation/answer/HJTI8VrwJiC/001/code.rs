// Answer 0

#[test]
fn test_byte_offset_initial_state() {
    struct TestIterator {
        data: Vec<u8>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Some(Ok(byte))
            } else {
                None
            }
        }
    }

    let iter = TestIterator { data: vec![], index: 0 };
    let line_col_iter = LineColIterator {
        iter,
        line: 1,
        col: 0,
        start_of_line: 0,
    };

    assert_eq!(line_col_iter.byte_offset(), 0);
}

#[test]
fn test_byte_offset_first_line() {
    struct TestIterator {
        data: Vec<u8>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Some(Ok(byte))
            } else {
                None
            }
        }
    }

    let data = b"Hello".to_vec();
    let iter = TestIterator { data, index: 0 };
    let mut line_col_iter = LineColIterator {
        iter,
        line: 1,
        col: 5,
        start_of_line: 0,
    };

    assert_eq!(line_col_iter.byte_offset(), 5);
}

#[test]
fn test_byte_offset_multiple_lines() {
    struct TestIterator {
        data: Vec<u8>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Some(Ok(byte))
            } else {
                None
            }
        }
    }

    let data = b"Hello\nWorld\n".to_vec();
    let iter = TestIterator { data, index: 0 };
    let mut line_col_iter = LineColIterator {
        iter,
        line: 2,
        col: 5,
        start_of_line: 6,
    };

    assert_eq!(line_col_iter.byte_offset(), 11);
}

#[test]
fn test_byte_offset_empty() {
    struct TestIterator {
        data: Vec<u8>,
        index: usize,
    }

    impl Iterator for TestIterator {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Some(Ok(byte))
            } else {
                None
            }
        }
    }

    let iter = TestIterator { data: vec![], index: 0 };
    let line_col_iter = LineColIterator {
        iter,
        line: 1,
        col: 0,
        start_of_line: 0,
    };

    assert_eq!(line_col_iter.byte_offset(), 0);
}

