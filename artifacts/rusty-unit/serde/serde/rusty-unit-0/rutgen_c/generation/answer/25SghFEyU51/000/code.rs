// Answer 0

#[test]
fn test_from_bounds_exact_size() {
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

    let iter = ExactSizeIter { count: 10 };
    assert_eq!(from_bounds(&iter), Some(10));
}

#[test]
fn test_from_bounds_non_exact_size() {
    struct NonExactSizeIter {
        count: usize,
    }

    impl Iterator for NonExactSizeIter {
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
            (self.count, None)
        }
    }

    let iter = NonExactSizeIter { count: 10 };
    assert_eq!(from_bounds(&iter), None);
}

#[test]
fn test_from_bounds_lower_bound_not_equal_upper_bound() {
    struct LowerNotEqualUpperIter {
        count: usize,
    }

    impl Iterator for LowerNotEqualUpperIter {
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
            (5, Some(10))
        }
    }

    let iter = LowerNotEqualUpperIter { count: 10 };
    assert_eq!(from_bounds(&iter), None);
}

#[test]
fn test_from_bounds_empty_iterator() {
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
    assert_eq!(from_bounds(&iter), Some(0));
}

