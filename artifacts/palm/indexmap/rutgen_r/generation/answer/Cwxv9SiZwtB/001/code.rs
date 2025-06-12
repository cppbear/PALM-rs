// Answer 0

#[test]
fn test_try_simplify_range_start_bound_unbounded() {
    use std::ops::{RangeBounds, Range, Bound};

    struct TestRange {
        inner: std::ops::RangeInclusive<usize>
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&10)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&5)
        }
    }

    let range = TestRange { inner: 0..=20 };
    let result = try_simplify_range(range, 5);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_start_bound_included() {
    use std::ops::{RangeBounds, Bound};

    struct TestRange {
        inner: std::ops::RangeInclusive<usize>
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&0)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&5)
        }
    }

    let range = TestRange { inner: 0..=20 };
    let result = try_simplify_range(range, 5);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_start_bound_excluded() {
    use std::ops::{RangeBounds, Bound};

    struct TestRange {
        inner: std::ops::RangeInclusive<usize>
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&7)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&5)
        }
    }

    let range = TestRange { inner: 0..=20 };
    let result = try_simplify_range(range, 5);
    assert_eq!(result, None);
}

