// Answer 0

#[test]
fn test_simplify_range_unbounded_start_and_end() {
    use std::ops::{Bound, RangeBounds};
    use std::iter::Range;

    struct Unbounded;

    impl RangeBounds<usize> for Unbounded {
        fn start_bound(&self) -> Bound<usize> {
            Bound::Unbounded
        }
        fn end_bound(&self) -> Bound<usize> {
            Bound::Unbounded
        }
    }

    let range = Unbounded;
    let len = 10;
    let result = simplify_range(range, len);
    assert_eq!(result, 0..10);
}

#[test]
fn test_simplify_range_included_start_and_excluded_end() {
    use std::ops::{Bound, RangeBounds};
    use std::iter::Range;

    struct IncludedExcluded;

    impl RangeBounds<usize> for IncludedExcluded {
        fn start_bound(&self) -> Bound<usize> {
            Bound::Included(&2)
        }
        fn end_bound(&self) -> Bound<usize> {
            Bound::Excluded(&8)
        }
    }

    let range = IncludedExcluded;
    let len = 10;
    let result = simplify_range(range, len);
    assert_eq!(result, 2..8);
}

#[test]
fn test_simplify_range_excluded_start_and_included_end() {
    use std::ops::{Bound, RangeBounds};
    use std::iter::Range;

    struct ExcludedIncluded;

    impl RangeBounds<usize> for ExcludedIncluded {
        fn start_bound(&self) -> Bound<usize> {
            Bound::Excluded(&3)
        }
        fn end_bound(&self) -> Bound<usize> {
            Bound::Included(&7)
        }
    }

    let range = ExcludedIncluded;
    let len = 10;
    let result = simplify_range(range, len);
    assert_eq!(result, 4..8);
}

#[test]
fn test_simplify_range_included_start_out_of_bounds() {
    use std::ops::{Bound, RangeBounds};
    
    struct IncludedOut;

    impl RangeBounds<usize> for IncludedOut {
        fn start_bound(&self) -> Bound<usize> {
            Bound::Included(&15)
        }
        fn end_bound(&self) -> Bound<usize> {
            Bound::Included(&5)
        }
    }

    let range = IncludedOut;
    let len = 10;
    let result = std::panic::catch_unwind(|| simplify_range(range, len));
    assert!(result.is_err());
}

#[test]
fn test_simplify_range_excluded_end_out_of_bounds() {
    use std::ops::{Bound, RangeBounds};
    
    struct ExcludedOut;

    impl RangeBounds<usize> for ExcludedOut {
        fn start_bound(&self) -> Bound<usize> {
            Bound::Excluded(&11)
        }
        fn end_bound(&self) -> Bound<usize> {
            Bound::Excluded(&14)
        }
    }

    let range = ExcludedOut;
    let len = 10;
    let result = std::panic::catch_unwind(|| simplify_range(range, len));
    assert!(result.is_err());
}

#[test]
fn test_simplify_range_start_greater_than_end() {
    use std::ops::{Bound, RangeBounds};

    struct StartGreaterThanEnd;

    impl RangeBounds<usize> for StartGreaterThanEnd {
        fn start_bound(&self) -> Bound<usize> {
            Bound::Included(&8)
        }
        fn end_bound(&self) -> Bound<usize> {
            Bound::Included(&5)
        }
    }

    let range = StartGreaterThanEnd;
    let len = 10;
    let result = std::panic::catch_unwind(|| simplify_range(range, len));
    assert!(result.is_err());
}

