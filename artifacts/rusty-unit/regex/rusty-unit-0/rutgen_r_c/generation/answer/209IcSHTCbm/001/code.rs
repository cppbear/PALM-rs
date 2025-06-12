// Answer 0

#[test]
fn test_end_valid_range() {
    struct TestInterval {
        start: char,
        end: char,
    }

    impl Interval for TestInterval {
        type Bound = char;

        fn lower(&self) -> Self::Bound {
            self.start
        }

        fn upper(&self) -> Self::Bound {
            self.end
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.start = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.end = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool {
            true
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let range = TestInterval { start: 'a', end: 'z' };
    assert_eq!(range.upper(), 'z');
}

#[test]
fn test_end_same_start_end() {
    struct TestInterval {
        start: char,
        end: char,
    }

    impl Interval for TestInterval {
        type Bound = char;

        fn lower(&self) -> Self::Bound {
            self.start
        }

        fn upper(&self) -> Self::Bound {
            self.end
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.start = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.end = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool {
            true
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let range = TestInterval { start: 'c', end: 'c' };
    assert_eq!(range.upper(), 'c');
}

#[test]
#[should_panic]
fn test_end_invalid_range() {
    struct TestInterval {
        start: char,
        end: char,
    }

    impl Interval for TestInterval {
        type Bound = char;

        fn lower(&self) -> Self::Bound {
            self.start
        }

        fn upper(&self) -> Self::Bound {
            self.end
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.start = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.end = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.end >= other.start
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.end < other.start || self.start > other.end
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.start >= other.start && self.end <= other.end
        }
    }

    let range = TestInterval { start: 'z', end: 'a' }; // Invalid range
    let _ = range.upper(); // This should not panic, but the structure is flawed
}

