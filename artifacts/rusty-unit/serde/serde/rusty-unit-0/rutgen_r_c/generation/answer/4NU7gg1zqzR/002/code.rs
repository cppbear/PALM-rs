// Answer 0

#[test]
fn test_iterator_len_hint_equal_size() {
    struct TestIter {
        count: usize,
    }

    impl Iterator for TestIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(self.count)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }

    let mut iter = TestIter { count: 5 };
    assert_eq!(iterator_len_hint(&iter), Some(5));
}

#[test]
fn test_iterator_len_hint_empty() {
    struct TestIterEmpty {
        count: usize,
    }

    impl Iterator for TestIterEmpty {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0))
        }
    }

    let iter = TestIterEmpty { count: 0 };
    assert_eq!(iterator_len_hint(&iter), Some(0));
}

