// Answer 0

#[test]
fn test_try_simplify_range_included_start_greater_than_len() {
    use std::ops::{Bound, Range, RangeBounds};

    struct InclusiveRange(usize, usize);

    impl RangeBounds<usize> for InclusiveRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.0)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.1)
        }
    }

    let range = InclusiveRange(5, 10);
    let len = 4;
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_excluded_start_greater_than_len() {
    use std::ops::{Bound, Range, RangeBounds};

    struct ExclusiveRange(usize, usize);

    impl RangeBounds<usize> for ExclusiveRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.0)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.1)
        }
    }

    let range = ExclusiveRange(5, 10);
    let len = 4;
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_included_start_and_excluded_end() {
    use std::ops::{Bound, RangeBounds};

    struct CombinedRange(usize, usize);

    impl RangeBounds<usize> for CombinedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.0)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.1)
        }
    }

    let range = CombinedRange(5, 10);
    let len = 4;
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

