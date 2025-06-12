// Answer 0

#[test]
#[should_panic(expected = "range start index 5 out of range for slice of length 5")]
fn test_simplify_range_start_index_out_of_bounds_included() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start.to_reference()
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end.to_reference()
        }
    }

    let range = TestRange {
        start: Bound::Included(&5),
        end: Bound::Excluded(&5),
    };
    
    let _result = simplify_range(range, 5);
}

#[test]
#[should_panic(expected = "range end index 5 out of range for slice of length 5")]
fn test_simplify_range_end_index_out_of_bounds_included() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start.to_reference()
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end.to_reference()
        }
    }

    let range = TestRange {
        start: Bound::Excluded(&4),
        end: Bound::Included(&5),
    };
    
    let _result = simplify_range(range, 5);
}

#[test]
#[should_panic(expected = "range start index 6 out of range for slice of length 5")]
fn test_simplify_range_start_index_out_of_bounds_excluded() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start.to_reference()
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end.to_reference()
        }
    }

    let range = TestRange {
        start: Bound::Excluded(&5),
        end: Bound::Excluded(&5),
    };
    
    let _result = simplify_range(range, 5);
}

#[test]
fn test_simplify_range_valid_case() {
    use std::ops::{Range, RangeBounds};
    use std::ops::Bound;

    struct TestRange {
        start: Bound<usize>,
        end: Bound<usize>,
    }

    impl RangeBounds<usize> for TestRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start.to_reference()
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end.to_reference()
        }
    }

    let range = TestRange {
        start: Bound::Excluded(&3),
        end: Bound::Excluded(&5),
    };

    let result = simplify_range(range, 5);
    assert_eq!(result, 4..5);
}

