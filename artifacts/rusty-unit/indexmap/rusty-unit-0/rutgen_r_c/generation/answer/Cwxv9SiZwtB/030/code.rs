// Answer 0

#[test]
fn test_try_simplify_range_included_start_not_meeting_length() {
    use core::ops::{Bound, RangeBounds};
    
    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }
    
    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start
        }
        fn end_bound(&self) -> Bound<&usize> {
            self.end
        }
    }
    
    let range = TestRange {
        start: Bound::Included(&5), // i = 5
        end: Bound::Unbounded,
    };
    let len = 4; // i <= len is false, since 5 > 4
    
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_included_end_not_meeting_length() {
    use core::ops::{Bound, RangeBounds};
    
    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }
    
    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start
        }
        fn end_bound(&self) -> Bound<&usize> {
            self.end
        }
    }
    
    let range = TestRange {
        start: Bound::Included(&3), // i = 3
        end: Bound::Included(&5), // i = 5
    };
    let len = 4; // i <= len is false for end, since 5 > 4
    
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_included_start_and_end_not_meeting_length() {
    use core::ops::{Bound, RangeBounds};
    
    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }
    
    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start
        }
        fn end_bound(&self) -> Bound<&usize> {
            self.end
        }
    }
    
    let range = TestRange {
        start: Bound::Included(&6), // i = 6
        end: Bound::Included(&7), // i = 7
    };
    let len = 5; // both start and end bounds exceed len
    
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

