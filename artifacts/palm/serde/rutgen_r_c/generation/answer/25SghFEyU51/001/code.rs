// Answer 0

#[test]
fn test_from_bounds_exact_size() {
    struct ExactSizeIter {
        current: usize,
        upper: usize,
    }

    impl Iterator for ExactSizeIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.upper {
                let result = Some(self.current);
                self.current += 1;
                result
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.upper - self.current, Some(self.upper - self.current))
        }
    }

    let iter = ExactSizeIter { current: 0, upper: 5 };
    assert_eq!(from_bounds(&iter), Some(5));
}

#[test]
fn test_from_bounds_range() {
    struct RangeIter {
        start: usize,
        end: usize,
        current: usize,
    }

    impl Iterator for RangeIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let result = Some(self.current);
                self.current += 1;
                result
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.end - self.current, Some(self.end - self.current))
        }
    }

    let iter = RangeIter { start: 0, end: 3, current: 0 };
    assert_eq!(from_bounds(&iter), Some(3));
}

#[test]
fn test_from_bounds_incomplete_size() {
    struct IncompleteSizeIter {
        current: usize,
        limit: usize,
    }

    impl Iterator for IncompleteSizeIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.limit {
                let result = Some(self.current);
                self.current += 1;
                result
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.limit - self.current, None) // No upper bound
        }
    }

    let iter = IncompleteSizeIter { current: 0, limit: 5 };
    assert_eq!(from_bounds(&iter), None);
}

#[test]
fn test_from_bounds_empty() {
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

#[test]
fn test_from_bounds_lower_upper_different() {
    struct LowerUpperDifferentIter {
        current: usize,
        upper: usize,
    }

    impl Iterator for LowerUpperDifferentIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.upper {
                let result = Some(self.current);
                self.current += 1;
                result
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.upper, Some(self.upper + 1)) // Different lower and upper
        }
    }

    let iter = LowerUpperDifferentIter { current: 0, upper: 5 };
    assert_eq!(from_bounds(&iter), None);
}

