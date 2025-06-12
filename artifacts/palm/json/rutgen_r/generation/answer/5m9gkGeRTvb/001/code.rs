// Answer 0

#[test]
fn test_new_line_col_iterator_with_valid_iter() {
    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                None
            } else {
                self.count -= 1;
                Some('a') // returning a dummy character
            }
        }
    }

    let iter = TestIterator { count: 5 };
    let line_col_iterator = new(iter);

    assert_eq!(line_col_iterator.line, 1);
    assert_eq!(line_col_iterator.col, 0);
    assert_eq!(line_col_iterator.start_of_line, 0);
}

#[test]
fn test_new_line_col_iterator_with_empty_iter() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let empty_iter = EmptyIterator;
    let line_col_iterator = new(empty_iter);

    assert_eq!(line_col_iterator.line, 1);
    assert_eq!(line_col_iterator.col, 0);
    assert_eq!(line_col_iterator.start_of_line, 0);
}

