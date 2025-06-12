// Answer 0

fn try_simplify_range_tests() {
    use std::ops::{Bound, RangeBounds, Range};

    struct InclusiveRange {
        start: usize,
        end: usize,
    }

    impl RangeBounds<usize> for InclusiveRange {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.start)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.end)
        }
    }

    // Test Case 1: Valid case with maximum inclusivity where start equals len.
    let range1 = InclusiveRange { start: 5, end: 10 };
    let len1 = 5;
    assert_eq!(try_simplify_range(range1, len1), Some(5..10));

    // Test Case 2: Edge case where inclusive start is equal to len.
    let range2 = InclusiveRange { start: 5, end: 8 };
    let len2 = 5;
    assert_eq!(try_simplify_range(range2, len2), None); // start = len, end < len

    // Test Case 3: Edge case where inclusive start is less than len.
    let range3 = InclusiveRange { start: 3, end: 8 };
    let len3 = 5;
    assert_eq!(try_simplify_range(range3, len3), Some(3..8));

    // Test Case 4: where inclusive start is greater than end which uses excluded.
    let range4 = InclusiveRange { start: 6, end: 5 };
    let len4 = 5;
    assert_eq!(try_simplify_range(range4, len4), None); // Start exceeds end

    // Test Case 5: Testing bound where start is included and end is excluded but higher than len.
    let range5 = InclusiveRange { start: 5, end: 7 };
    let len5 = 6;
    assert_eq!(try_simplify_range(range5, len5), None); // start = len, end is excluded but out of bounds

    // Test Case 6: Valid case with includes and bounds not exceeding.
    let range6 = InclusiveRange { start: 2, end: 5 };
    let len6 = 5;
    assert_eq!(try_simplify_range(range6, len6), Some(2..5));
}

