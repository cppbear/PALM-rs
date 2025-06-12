// Answer 0

#[test]
fn test_line_col_iterator_new() {
    struct MockIterator {
        data: Vec<u8>,
        index: usize,
    }
    
    impl Iterator for MockIterator {
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

    let mock_data = vec![b'a', b'b', b'\n', b'c', b'd'];
    let mock_iter = MockIterator { data: mock_data, index: 0 };
    let line_col_iterator = LineColIterator::new(mock_iter);

    assert_eq!(line_col_iterator.line, 1);
    assert_eq!(line_col_iterator.col, 0);
    assert_eq!(line_col_iterator.start_of_line, 0);
}

