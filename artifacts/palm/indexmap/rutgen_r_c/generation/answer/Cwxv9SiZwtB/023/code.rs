// Answer 0

#[test]
fn test_range_with_bound_included_at_len() {
    use core::ops::{Bound, Range, RangeBounds};

    struct IncludedRange(usize);
    
    impl RangeBounds<usize> for IncludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.0)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }

    let range = IncludedRange(5);
    let len = 5;
    let result = try_simplify_range(range, len);
    assert_eq!(result, Some(5..5));
}

#[test]
fn test_range_with_bound_included_below_len() {
    use core::ops::{Bound, Range, RangeBounds};

    struct IncludedRange(usize);
    
    impl RangeBounds<usize> for IncludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.0)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }

    let range = IncludedRange(3);
    let len = 5;
    let result = try_simplify_range(range, len);
    assert_eq!(result, Some(3..5));
}

#[test]
fn test_range_with_excluded_below_len() {
    use core::ops::{Bound, Range, RangeBounds};

    struct ExcludedRange(usize);
    
    impl RangeBounds<usize> for ExcludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.0)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }

    let range = ExcludedRange(2);
    let len = 5;
    let result = try_simplify_range(range, len);
    assert_eq!(result, Some(3..5));
}

#[test]
fn test_range_with_unbounded_start_and_unbounded_end() {
    use core::ops::{Bound, Range, RangeBounds};

    struct UnboundedRange;
    
    impl RangeBounds<usize> for UnboundedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }

    let range = UnboundedRange;
    let len = 10;
    let result = try_simplify_range(range, len);
    assert_eq!(result, Some(0..10));
}

#[test]
fn test_range_with_start_excluded_equal_len() {
    use core::ops::{Bound, Range, RangeBounds};

    struct ExcludedRange(usize);
    
    impl RangeBounds<usize> for ExcludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.0)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.0)
        }
    }

    let range = ExcludedRange(5);
    let len = 5;
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

