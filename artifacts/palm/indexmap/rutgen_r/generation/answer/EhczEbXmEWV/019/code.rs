// Answer 0

#[test]
fn test_simplify_range_with_included_start_and_unbounded_end() {
    use std::ops::{Bound, RangeBounds};
    
    struct IncludedStartRange {
        start: usize,
    }
    
    impl RangeBounds<usize> for IncludedStartRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }
    
    let len = 5;
    let range = IncludedStartRange { start: 5 }; // i == len
    let result = simplify_range(range, len);
    assert_eq!(result, 5..5);
}

#[test]
#[should_panic(expected = "range start index 6 out of range for slice of length 5")]
fn test_simplify_range_with_included_start_out_of_range() {
    use std::ops::{Bound, RangeBounds};
    
    struct IncludedStartRange {
        start: usize,
    }
    
    impl RangeBounds<usize> for IncludedStartRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Unbounded
        }
    }
    
    let len = 5;
    let range = IncludedStartRange { start: 6 }; // i > len
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range end index 5 out of range for slice of length 5")]
fn test_simplify_range_with_excluded_end_out_of_range() {
    use std::ops::{Bound, RangeBounds};
    
    struct ExcludedEndRange {
        end: usize,
    }
    
    impl RangeBounds<usize> for ExcludedEndRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&0)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.end)
        }
    }
    
    let len = 5;
    let range = ExcludedEndRange { end: 5 }; // i == len
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range start index 6 out of range for slice of length 5")]
fn test_simplify_range_with_excluded_start_out_of_range() {
    use std::ops::{Bound, RangeBounds};
    
    struct ExcludedStartRange {
        start: usize,
    }
    
    impl RangeBounds<usize> for ExcludedStartRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.start)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&5)
        }
    }
    
    let len = 5;
    let range = ExcludedStartRange { start: 6 }; // i > len
    simplify_range(range, len);
}

#[test]
#[should_panic(expected = "range start index 5 should be <= range end index 6")]
fn test_simplify_range_with_start_greater_than_end() {
    use std::ops::{Bound, RangeBounds};
    
    struct InvalidRange {
        start: usize,
        end: usize,
    }
    
    impl RangeBounds<usize> for InvalidRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.end)
        }
    }
    
    let len = 6;
    let range = InvalidRange { start: 6, end: 5 }; // start > end
    simplify_range(range, len);
}

