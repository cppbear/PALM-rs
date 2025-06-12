// Answer 0

#[test]
fn test_try_simplify_range_included() {
    use std::ops::{Range, Bound, RangeBounds};

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end
        }
    }

    let range = TestRange {
        start: Bound::Included(&2),
        end: Bound::Excluded(&5),
    };

    let result = try_simplify_range(range, 5);
    assert_eq!(result, Some(2..5));
}

#[test]
fn test_try_simplify_range_excluded() {
    use std::ops::{Range, Bound, RangeBounds};

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end
        }
    }

    let range = TestRange {
        start: Bound::Excluded(&2),
        end: Bound::Included(&5),
    };

    let result = try_simplify_range(range, 5);
    assert_eq!(result, Some(3..6));
}

#[test]
fn test_try_simplify_range_unbounded() {
    use std::ops::{Range, Bound, RangeBounds};

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end
        }
    }

    let range = TestRange {
        start: Bound::Unbounded,
        end: Bound::Unbounded,
    };

    let result = try_simplify_range(range, 10);
    assert_eq!(result, Some(0..10));
}

#[test]
fn test_try_simplify_range_none() {
    use std::ops::{Range, Bound, RangeBounds};

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end
        }
    }

    let range = TestRange {
        start: Bound::Included(&10),
        end: Bound::Excluded(&5),
    };

    let result = try_simplify_range(range, 5);
    assert_eq!(result, None);
}

