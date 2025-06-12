// Answer 0

#[test]
fn test_iterator_len_hint_exact_length() {
    struct ExactLengthIter {
        count: usize,
    }

    impl Iterator for ExactLengthIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }

    let iter = ExactLengthIter { count: 5 };
    assert_eq!(iterator_len_hint(&iter), Some(5));
}

#[test]
fn test_iterator_len_hint_inexact_length() {
    struct InexactLengthIter {
        count: usize,
    }

    impl Iterator for InexactLengthIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, None)
        }
    }

    let iter = InexactLengthIter { count: 5 };
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
            (0, Some(0))
        }
    }

    let iter = EmptyIter;
    assert_eq!(iterator_len_hint(&iter), Some(0));
}

#[test]
fn test_iterator_len_hint_halved() {
    struct HalfLengthIter {
        count: usize,
    }

    impl Iterator for HalfLengthIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(1)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count / 2, Some(self.count))
        }
    }

    let iter = HalfLengthIter { count: 6 };
    assert_eq!(iterator_len_hint(&iter), None);
}

