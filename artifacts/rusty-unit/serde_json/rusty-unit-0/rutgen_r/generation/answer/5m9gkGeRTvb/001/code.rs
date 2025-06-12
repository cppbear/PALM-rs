// Answer 0

#[test]
fn test_new_with_valid_iterator() {
    struct ValidIterator {
        values: Vec<i32>,
        index: usize,
    }

    impl Iterator for ValidIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Some(value)
            } else {
                None
            }
        }
    }

    let iter = ValidIterator {
        values: vec![1, 2, 3],
        index: 0,
    };

    let line_col_iterator = new(iter);
    assert_eq!(line_col_iterator.line, 1);
    assert_eq!(line_col_iterator.col, 0);
    assert_eq!(line_col_iterator.start_of_line, 0);
}

#[test]
fn test_new_with_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let iter = EmptyIterator;
    let line_col_iterator = new(iter);
    assert_eq!(line_col_iterator.line, 1);
    assert_eq!(line_col_iterator.col, 0);
    assert_eq!(line_col_iterator.start_of_line, 0);
}

#[test]
#[should_panic]
fn test_new_with_panicking_iterator() {
    struct PanickingIterator;

    impl Iterator for PanickingIterator {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            panic!("This iterator always panics");
        }
    }

    let iter = PanickingIterator;
    let _line_col_iterator = new(iter); // This should panic
}

