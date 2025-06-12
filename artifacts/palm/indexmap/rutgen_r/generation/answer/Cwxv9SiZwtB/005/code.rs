// Answer 0

fn try_simplify_range_tests() {
    use std::ops::{Bound, Range, RangeBounds};

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

    #[test]
    fn test_unbounded_range_start_with_excluded_end() {
        let range = TestRange {
            start: Bound::Unbounded,
            end: Bound::Excluded(&5),
        };
        let len = 5;
        assert_eq!(try_simplify_range(range, len), Some(0..5));
    }

    #[test]
    fn test_unbounded_range_start_with_included_end() {
        let range = TestRange {
            start: Bound::Unbounded,
            end: Bound::Included(&5),
        };
        let len = 5;
        assert_eq!(try_simplify_range(range, len), Some(0..6));
    }

    #[test]
    fn test_excluded_start_at_len_and_excluded_end_at_len() {
        let range = TestRange {
            start: Bound::Excluded(&5),
            end: Bound::Excluded(&5),
        };
        let len = 5;
        assert_eq!(try_simplify_range(range, len), None);
    }

    #[test]
    fn test_included_start_at_len_and_excluded_end_at_len() {
        let range = TestRange {
            start: Bound::Included(&5),
            end: Bound::Excluded(&5),
        };
        let len = 5;
        assert_eq!(try_simplify_range(range, len), None);
    }

    #[test]
    fn test_excluded_start_greater_than_end() {
        let range = TestRange {
            start: Bound::Excluded(&6),
            end: Bound::Excluded(&5),
        };
        let len = 5;
        assert_eq!(try_simplify_range(range, len), None);
    }
}

