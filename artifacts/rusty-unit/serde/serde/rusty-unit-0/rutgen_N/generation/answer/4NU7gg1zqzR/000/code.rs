// Answer 0

#[test]
fn test_iterator_len_hint_exact_size() {
    struct ExactSizeIter {
        count: usize,
    }

    impl Iterator for ExactSizeIter {
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

    let iter = ExactSizeIter { count: 5 };
    assert_eq!(iterator_len_hint(&iter), Some(5));
}

#[test]
fn test_iterator_len_hint_variable_size() {
    struct VariableSizeIter {
        count: usize,
    }

    impl Iterator for VariableSizeIter {
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
            (self.count, None) // upper bound is unknown
        }
    }

    let iter = VariableSizeIter { count: 3 };
    assert_eq!(iterator_len_hint(&iter), None);
}

#[test]
fn test_iterator_len_hint_empty() {
    struct EmptyIter;

    impl Iterator for EmptyIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0)) // exact size of 0
        }
    }

    let iter = EmptyIter;
    assert_eq!(iterator_len_hint(&iter), Some(0));
}

