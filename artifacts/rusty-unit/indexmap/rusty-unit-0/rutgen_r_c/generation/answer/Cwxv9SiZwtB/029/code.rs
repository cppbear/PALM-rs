// Answer 0

#[test]
fn test_try_simplify_range_start_bound_included_and_end_bound_included_equal_len() {
    use core::ops::{Bound, Range, RangeBounds};

    struct TestRange {
        bounds: Range<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.bounds.start)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.bounds.end)
        }
    }

    let range = TestRange { bounds: 5..5 }; // i == len
    let len = 5;
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_start_bound_included_altered_len() {
    use core::ops::{Bound, Range, RangeBounds};

    struct TestRange {
        bounds: Range<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.bounds.start)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.bounds.end)
        }
    }

    let range = TestRange { bounds: 5..5 }; // i == len
    let len = 4; // len is less than i
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

