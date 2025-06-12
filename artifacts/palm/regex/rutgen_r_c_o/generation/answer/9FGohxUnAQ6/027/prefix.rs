// Answer 0

#[test]
fn test_difference_non_empty_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl Bound for TestBound {
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }

        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct TestInterval {
        lower_bound: TestBound,
        upper_bound: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower_bound
        }

        fn upper(&self) -> Self::Bound {
            self.upper_bound
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower_bound = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper_bound = bound;
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

    let mut set_a = IntervalSet::new(vec![TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(5) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower_bound: TestBound(3), upper_bound: TestBound(4) }]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_empty_self() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl Bound for TestBound {
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }

        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct TestInterval {
        lower_bound: TestBound,
        upper_bound: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower_bound
        }

        fn upper(&self) -> Self::Bound {
            self.upper_bound
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower_bound = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper_bound = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            true
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut set_a = IntervalSet::new(vec![]);
    let set_b = IntervalSet::new(vec![TestInterval { lower_bound: TestBound(3), upper_bound: TestBound(4) }]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_empty_other() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl Bound for TestBound {
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }

        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct TestInterval {
        lower_bound: TestBound,
        upper_bound: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower_bound
        }

        fn upper(&self) -> Self::Bound {
            self.upper_bound
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower_bound = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper_bound = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            true
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut set_a = IntervalSet::new(vec![TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(5) }]);
    let set_b = IntervalSet::new(vec![]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_with_non_contiguous_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl Bound for TestBound {
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }

        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct TestInterval {
        lower_bound: TestBound,
        upper_bound: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower_bound
        }

        fn upper(&self) -> Self::Bound {
            self.upper_bound
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower_bound = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper_bound = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut set_a = IntervalSet::new(vec![TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(5) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower_bound: TestBound(6), upper_bound: TestBound(8) }]);

    set_a.difference(&set_b);
}

