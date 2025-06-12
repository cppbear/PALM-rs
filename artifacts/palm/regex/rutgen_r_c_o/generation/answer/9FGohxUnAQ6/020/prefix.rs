// Answer 0

#[test]
fn test_difference_non_empty_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl Bound for TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower
        }

        fn upper(&self) -> Self::Bound {
            self.upper
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower && self.lower <= other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(5),
        },
    ]);

    let interval_set_b = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(5),
            upper: TestBound(10),
        },
    ]);

    interval_set_a.difference(&interval_set_b);
}

#[test]
fn test_difference_non_empty_partial_overlap() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl Bound for TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower
        }

        fn upper(&self) -> Self::Bound {
            self.upper
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower && self.lower <= other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(6),
        },
    ]);

    let interval_set_b = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(5),
            upper: TestBound(10),
        },
    ]);

    interval_set_a.difference(&interval_set_b);
}

#[test]
fn test_difference_non_empty_full_overlap() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl Bound for TestBound {
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower
        }

        fn upper(&self) -> Self::Bound {
            self.upper
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower && self.lower <= other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(10),
        },
    ]);

    let interval_set_b = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(10),
        },
    ]);

    interval_set_a.difference(&interval_set_b);
}

