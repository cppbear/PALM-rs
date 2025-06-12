// Answer 0

#[test]
fn test_try_simplify_range_excluded_start_included_end() {
    use core::ops::{Bound, RangeBounds};

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

    let len = 5;

    // Test case: Excluded start and Included end
    let range = TestRange {
        start: Bound::Excluded(&2),
        end: Bound::Included(&4),
    };

    assert_eq!(try_simplify_range(range, len), Some(3..5));
}

#[test]
fn test_try_simplify_range_excluded_start_excluded_end() {
    use core::ops::{Bound, RangeBounds};

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

    let len = 5;

    // Test case: Excluded start and Excluded end
    let range = TestRange {
        start: Bound::Excluded(&1),
        end: Bound::Excluded(&4),
    };

    assert_eq!(try_simplify_range(range, len), Some(2..4));
}

#[test]
fn test_try_simplify_range_included_start_included_end() {
    use core::ops::{Bound, RangeBounds};

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

    let len = 5;

    // Test case: Included start and Included end
    let range = TestRange {
        start: Bound::Included(&2),
        end: Bound::Included(&3),
    };

    assert_eq!(try_simplify_range(range, len), Some(2..4));
}

#[test]
fn test_try_simplify_range_excluded_start_unbounded_end() {
    use core::ops::{Bound, RangeBounds};

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

    let len = 5;

    // Test case: Excluded start and Unbounded end
    let range = TestRange {
        start: Bound::Excluded(&1),
        end: Bound::Unbounded,
    };

    assert_eq!(try_simplify_range(range, len), Some(2..5));
}

#[test]
fn test_try_simplify_range_unbounded_start_excluded_end() {
    use core::ops::{Bound, RangeBounds};

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

    let len = 5;

    // Test case: Unbounded start and Excluded end
    let range = TestRange {
        start: Bound::Unbounded,
        end: Bound::Excluded(&5),
    };

    assert_eq!(try_simplify_range(range, len), Some(0..5));
}

#[test]
fn test_try_simplify_range_unbounded_start_included_end() {
    use core::ops::{Bound, RangeBounds};

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

    let len = 5;

    // Test case: Unbounded start and Included end
    let range = TestRange {
        start: Bound::Unbounded,
        end: Bound::Included(&3),
    };

    assert_eq!(try_simplify_range(range, len), Some(0..4));
}

