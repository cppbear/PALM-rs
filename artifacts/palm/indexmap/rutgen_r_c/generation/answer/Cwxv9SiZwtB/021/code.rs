// Answer 0

#[test]
fn test_try_simplify_range_inclusive_start_equal_length() {
    use core::ops::RangeBounds;
    use core::ops::Bound;

    struct InclusiveStart {
        value: usize,
    }

    impl RangeBounds<usize> for InclusiveStart {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.value)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&(self.value + 1)) // This should trigger the None
        }
    }

    let range = InclusiveStart { value: 5 };
    let len = 5;
    assert_eq!(try_simplify_range(range, len), None);
}

#[test]
fn test_try_simplify_range_inclusive_start_and_exclusive_end_greater_len() {
    use core::ops::RangeBounds;
    use core::ops::Bound;

    struct InclusiveStart {
        value: usize,
    }

    impl RangeBounds<usize> for InclusiveStart {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.value)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&(self.value + 2)) // This should trigger the None
        }
    }

    let range = InclusiveStart { value: 5 };
    let len = 6;
    assert_eq!(try_simplify_range(range, len), None);
}

#[test]
fn test_try_simplify_range_inclusive_start_and_exclusive_end_equal_len() {
    use core::ops::RangeBounds;
    use core::ops::Bound;

    struct InclusiveStart {
        value: usize,
    }

    impl RangeBounds<usize> for InclusiveStart {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.value)
        }
        fn end_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.value) // This should trigger the None
        }
    }

    let range = InclusiveStart { value: 3 };
    let len = 3;
    assert_eq!(try_simplify_range(range, len), None);
}

