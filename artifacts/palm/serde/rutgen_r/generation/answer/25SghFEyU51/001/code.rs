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
                Some(1)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }

    let iter = ExactSizeIter { count: 5 };
    assert_eq!(serde::from_bounds(&iter), Some(5));
}

#[test]
fn test_from_bounds_range() {
    struct RangeIter {
        start: usize,
        end: usize,
    }

    impl Iterator for RangeIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.start < self.end {
                let val = self.start;
                self.start += 1;
                Some(val)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.end - self.start, Some(self.end - self.start))
        }
    }

    let iter = RangeIter { start: 0, end: 10 };
    assert_eq!(serde::from_bounds(&iter), Some(10));
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
    assert_eq!(serde::from_bounds(&iter), Some(0));
}

#[test]
fn test_from_bounds_infinite() {
    struct InfiniteIter;

    impl Iterator for InfiniteIter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            Some(1)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (usize::MAX, None)
        }
    }

    let iter = InfiniteIter;
    assert_eq!(serde::from_bounds(&iter), Some(usize::MAX));
}

