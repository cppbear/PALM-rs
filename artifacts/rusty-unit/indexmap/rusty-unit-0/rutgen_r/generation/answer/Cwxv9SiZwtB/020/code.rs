// Answer 0

#[test]
fn test_try_simplify_range_excluded_start_bound_equals_len() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            &self.start
        }

        fn end_bound(&self) -> Bound<&usize> {
            &self.end
        }
    }

    let len = 5;
    let range = TestRange {
        start: Bound::Excluded(&5),  // i == len
        end: Bound::Excluded(&5),    // i == len
    };
    
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

#[test]
fn test_try_simplify_range_included_start_bound_equals_len() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            &self.start
        }

        fn end_bound(&self) -> Bound<&usize> {
            &self.end
        }
    }

    let len = 5;
    let range = TestRange {
        start: Bound::Included(&5),  // i == len
        end: Bound::Excluded(&6),     // i > len
    };
    
    let result = try_simplify_range(range, len);
    assert_eq!(result, None);
}

