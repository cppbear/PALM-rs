// Answer 0

#[test]
fn test_from_bounds_with_non_empty_iterator() {
    struct TestIterator {
        current: usize,
        end: usize,
    }

    impl Iterator for TestIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.end - self.current, Some(self.end - self.current))
        }
    }

    let iter = TestIterator { current: 0, end: 10 };
    assert_eq!(serde::de::from_bounds(&iter), Some(10));
}

#[test]
fn test_from_bounds_with_empty_iterator() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0))
        }
    }

    let iter = EmptyIterator;
    assert_eq!(serde::de::from_bounds(&iter), Some(0));
}

#[test]
fn test_from_bounds_with_exceeding_bounds_iterator() {
    struct OverIterator {
        current: usize,
        end: usize,
    }

    impl Iterator for OverIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let value = self.current;
                self.current += 1;
                Some(value)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.end + 5, None) // Exceeding bounds
        }
    }

    let iter = OverIterator { current: 0, end: 10 };
    assert_eq!(serde::de::from_bounds(&iter), Some(15)); // Expected size hint should consider the None as unlimited
}

