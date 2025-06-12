// Answer 0

#[test]
fn test_difference_non_overlapping_ranges() {
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    impl Debug for TestBound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl Bound for TestBound {}
    
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
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

    let mut intervals_a = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    let intervals_b = IntervalSet::new(vec![TestInterval { lower: TestBound(2), upper: TestBound(3) }]);
    
    intervals_a.difference(&intervals_b);
}

#[test]
fn test_difference_partially_overlapping_ranges() {
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    impl Debug for TestBound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl Bound for TestBound {}
    
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
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

    let mut intervals_a = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    let intervals_b = IntervalSet::new(vec![TestInterval { lower: TestBound(4), upper: TestBound(6) }]);
    
    intervals_a.difference(&intervals_b);
}

#[test]
fn test_difference_completely_overlapping_ranges() {
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    impl Debug for TestBound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl Bound for TestBound {}
    
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
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

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            true
        }
    }

    let mut intervals_a = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    let intervals_b = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    
    intervals_a.difference(&intervals_b);
}

