// Answer 0

#[test]
fn test_try_simplify_range_none_bounds_included_false() {
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

    // Case: start is Excluded and end is Included
    let range = TestRange {
        start: Bound::Excluded(&5),
        end: Bound::Included(&10),
    };

    let result = try_simplify_range(range, 20);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_none_bounds_unbounded_false() {
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

    // Case: start is Included and end is Unbounded
    let range = TestRange {
        start: Bound::Included(&15),
        end: Bound::Unbounded,
    };

    let result = try_simplify_range(range, 20);
    assert_eq!(result, Some(15..20));
}

#[test]
fn test_try_simplify_range_none_bounds_excluded_false() {
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

    // Case: start is Unbounded and end is Excluded
    let range = TestRange {
        start: Bound::Unbounded,
        end: Bound::Excluded(&20),
    };

    let result = try_simplify_range(range, 20);
    assert_eq!(result, Some(0..20));
}

#[test]
fn test_try_simplify_range_none() {
    use core::ops::{Bound, RangeBounds};

    // Test when both start and end bounds are not satisfying any criteria
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

    // Example: start is Excluded and end is Included but values cause failure
    let range = TestRange {
        start: Bound::Excluded(&10), 
        end: Bound::Excluded(&5),
    };

    let result = try_simplify_range(range, 20);
    assert_eq!(result, None);
}

