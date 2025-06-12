// Answer 0

#[test]
fn test_try_simplify_range_excluded_included_equal() {
    use std::ops::{Range, Bound, RangeBounds};
    
    struct TestRange {
        range: Range<usize>,
    }
    
    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.range.start)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.range.end)
        }
    }
    
    let len = 5;
    let test_range = TestRange { range: 3..5 };
    let result = try_simplify_range(test_range, len);
    assert_eq!(result, Some(3..5));
}

#[test]
fn test_try_simplify_range_excluded_excluded_equal() {
    use std::ops::{Range, Bound, RangeBounds};
    
    struct TestRange {
        range: Range<usize>,
    }
    
    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.range.start)
        }
        
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.range.end)
        }
    }
    
    let len = 5;
    let test_range = TestRange { range: 3..4 };
    let result = try_simplify_range(test_range, len);
    assert_eq!(result, Some(3..4));
}

