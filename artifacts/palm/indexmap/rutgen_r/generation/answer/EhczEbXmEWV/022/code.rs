// Answer 0

#[test]
fn test_simplify_range_included_start_bound() {
    use std::ops::{Range, Bound, RangeBounds};

    #[derive(Debug)]
    struct IncludedRange {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for IncludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.end)
        }
    }

    let range = IncludedRange { start: 0, end: 5 };
    let result = simplify_range(range, 5);
    assert_eq!(result, 0..6);
}

#[test]
fn test_simplify_range_excluded_end_bound() {
    use std::ops::{Range, Bound, RangeBounds};

    #[derive(Debug)]
    struct ExcludedRange {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for ExcludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.end)
        }
    }

    let range = ExcludedRange { start: 1, end: 5 };
    let result = simplify_range(range, 5);
    assert_eq!(result, 1..5);
}

#[test]
fn test_simplify_range_unbounded_start_bound() {
    use std::ops::{Range, Bound, RangeBounds};

    #[derive(Debug)]
    struct UnboundedStart {
        end: usize,
    }

    impl RangeBounds<usize> for UnboundedStart {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.end)
        }
    }

    let range = UnboundedStart { end: 3 };
    let result = simplify_range(range, 5);
    assert_eq!(result, 0..3);
}

#[test]
#[should_panic(expected = "range start index 5 out of range for slice of length 5")]
fn test_simplify_range_included_start_out_of_range() {
    use std::ops::{RangeBounds, Bound};

    #[derive(Debug)]
    struct OutOfRangeStart {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for OutOfRangeStart {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.end)
        }
    }

    let range = OutOfRangeStart { start: 5, end: 5 };
    let _ = simplify_range(range, 5);
}

#[test]
#[should_panic(expected = "range end index 5 out of range for slice of length 5")]
fn test_simplify_range_included_end_out_of_range() {
    use std::ops::{RangeBounds, Bound};

    #[derive(Debug)]
    struct OutOfRangeEnd {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for OutOfRangeEnd {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.end)
        }
    }

    let range = OutOfRangeEnd { start: 4, end: 5 };
    let _ = simplify_range(range, 5);
}

#[test]
#[should_panic(expected = "range start index 5 should be <= range end index 5")]
fn test_simplify_range_start_greater_than_end() {
    use std::ops::{RangeBounds, Bound};

    #[derive(Debug)]
    struct StartGreaterThanEnd {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for StartGreaterThanEnd {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.end)
        }
    }

    let range = StartGreaterThanEnd { start: 5, end: 5 };
    let _ = simplify_range(range, 5);
}

