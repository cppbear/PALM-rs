// Answer 0

#[test]
fn test_line_col_iterator_new_empty_iterator() {
    struct EmptyIter;
    impl Iterator for EmptyIter {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let iter = EmptyIter;
    let line_col_iterator = LineColIterator::new(iter);
    assert_eq!(line_col_iterator.line, 1);
    assert_eq!(line_col_iterator.col, 0);
    assert_eq!(line_col_iterator.start_of_line, 0);
}

#[test]
fn test_line_col_iterator_new_single_element_iterator() {
    struct SingleElementIter {
        count: usize,
    }

    impl Iterator for SingleElementIter {
        type Item = io::Result<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(Ok(b'a'))
            } else {
                None
            }
        }
    }

    let iter = SingleElementIter { count: 1 };
    let line_col_iterator = LineColIterator::new(iter);
    assert_eq!(line_col_iterator.line, 1);
    assert_eq!(line_col_iterator.col, 0);
    assert_eq!(line_col_iterator.start_of_line, 0);
}

#[test]
fn test_line_col_iterator_new_multiple_elements_iterator() {
    struct MultiElementIter {
        data: Vec<u8>,
        index: usize,
    }

    impl Iterator for MultiElementIter {
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

    let iter = MultiElementIter {
        data: b"hello\nworld".to_vec(),
        index: 0,
    };
    let line_col_iterator = LineColIterator::new(iter);
    assert_eq!(line_col_iterator.line, 1);
    assert_eq!(line_col_iterator.col, 0);
    assert_eq!(line_col_iterator.start_of_line, 0);
}

