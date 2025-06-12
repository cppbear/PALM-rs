// Answer 0

#[test]
fn test_try_simplify_range_included_start_unbounded_end() {
    use std::ops::{Range, RangeBounds, Bound};
    
    struct IncludedRange {
        start: usize,
    }
    
    impl RangeBounds<usize> for IncludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }

    let length = 10; // setting len to 10
    let range = IncludedRange { start: length }; // i == len
    let result = try_simplify_range(range, length);
    assert_eq!(result, Some(length..length)); // expect Some(10..10)
}

#[test]
fn test_try_simplify_range_included_start_included_end() {
    use std::ops::{Range, RangeBounds, Bound};
    
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

    let length = 5; // setting len to 5
    let range = IncludedRange { start: 5, end: 5 }; // i == len
    let result = try_simplify_range(range, length);
    assert_eq!(result, Some(5..6)); // expect Some(5..6)
}

#[test]
fn test_try_simplify_range_included_start_excluded_end() {
    use std::ops::{Range, RangeBounds, Bound};
    
    struct IncludedRange {
        start: usize,
    }
    
    impl RangeBounds<usize> for IncludedRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&(self.start + 1))
        }
    }

    let length = 7; // setting len to 7
    let range = IncludedRange { start: 6 }; // i < len
    let result = try_simplify_range(range, length);
    assert_eq!(result, Some(6..7)); // expect Some(6..7)
}

