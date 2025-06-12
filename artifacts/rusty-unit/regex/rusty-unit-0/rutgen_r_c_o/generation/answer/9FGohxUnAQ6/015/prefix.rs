// Answer 0

#[test]
fn test_difference_no_overlap() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);
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

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}
        
        fn is_contiguous(&self, other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            true
        }

        fn is_subset(&self, other: &Self) -> bool {
            false
        }
    }

    let mut set1 = IntervalSet::new(vec![
        TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(3) },
        TestInterval { lower_bound: TestBound(5), upper_bound: TestBound(7) },
    ]);

    let set2 = IntervalSet::new(vec![
        TestInterval { lower_bound: TestBound(8), upper_bound: TestBound(10) },
    ]);

    set1.difference(&set2);
}

#[test]
fn test_difference_some_overlap() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);
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

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}
        
        fn is_contiguous(&self, other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper_bound.0 < other.lower_bound.0
        }

        fn is_subset(&self, other: &Self) -> bool {
            false
        }
    }

    let mut set1 = IntervalSet::new(vec![
        TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(5) },
    ]);

    let set2 = IntervalSet::new(vec![
        TestInterval { lower_bound: TestBound(3), upper_bound: TestBound(4) },
    ]);

    set1.difference(&set2);
}

#[test]
fn test_difference_complete_overlap() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);
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

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}
        
        fn is_contiguous(&self, other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            false
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower_bound.0 >= other.lower_bound.0 && self.upper_bound.0 <= other.upper_bound.0
        }
    }

    let mut set1 = IntervalSet::new(vec![
        TestInterval { lower_bound: TestBound(2), upper_bound: TestBound(5) },
    ]);

    let set2 = IntervalSet::new(vec![
        TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(6) },
    ]);

    set1.difference(&set2);
}

