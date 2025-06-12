// Answer 0

#[test]
fn test_try_simplify_range_excluded_start_bound_equal_length() {
    use core::ops::{RangeBounds, Bound};

    struct ExcludedRange {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for ExcludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.start)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.end)
        }
    }

    let range = ExcludedRange { start: 5, end: 5 };
    let len = 5;
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_excluded_end_bound_equal_length() {
    use core::ops::{RangeBounds, Bound};

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

    let range = ExcludedRange { start: 5, end: 5 };
    let len = 5;
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

