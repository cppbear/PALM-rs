// Answer 0

#[test]
fn test_iterator_len_hint_exact_size() {
    struct TestIterator {
        current: usize,
        limit: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.limit {
                self.current += 1;
                Some(self.current - 1)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.limit, Some(self.limit))
        }
    }

    let iter = TestIterator { current: 0, limit: 5 };
    let result = iterator_len_hint(&iter);
    assert_eq!(result, Some(5));
}

#[test]
fn test_iterator_len_hint_zero_size() {
    struct TestIterator {
        current: usize,
        limit: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.limit {
                self.current += 1;
                Some(self.current - 1)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0))
        }
    }

    let iter = TestIterator { current: 0, limit: 0 };
    let result = iterator_len_hint(&iter);
    assert_eq!(result, Some(0));
}

