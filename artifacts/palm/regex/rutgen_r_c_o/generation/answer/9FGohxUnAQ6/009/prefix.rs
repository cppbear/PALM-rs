// Answer 0

#[test]
fn test_difference_with_edge_case() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);
    impl Bound for TestBound {}

    #[derive(Clone, Debug, Eq, PartialEq)]
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
            self.upper.0 >= other.lower.0
        }
        
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0
        }
        
        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
        TestInterval { lower: TestBound(7), upper: TestBound(10) },
    ]);
    
    let interval_set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(5), upper: TestBound(10) },
        TestInterval { lower: TestBound(11), upper: TestBound(12) },
    ]);
    
    interval_set_a.difference(&interval_set_b);
}

#[test]
fn test_difference_with_non_overlapping_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);
    impl Bound for TestBound {}

    #[derive(Clone, Debug, Eq, PartialEq)]
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
            self.upper.0 >= other.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(6), upper: TestBound(8) },
    ]);

    let interval_set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(4), upper: TestBound(5) },
        TestInterval { lower: TestBound(9), upper: TestBound(10) },
    ]);

    interval_set_a.difference(&interval_set_b);
}

#[test]
fn test_difference_with_adjacent_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u32);
    impl Bound for TestBound {}

    #[derive(Clone, Debug, Eq, PartialEq)]
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
            self.upper.0 >= other.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
    }

    let mut interval_set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
        TestInterval { lower: TestBound(6), upper: TestBound(10) },
    ]);

    let interval_set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(5), upper: TestBound(6) },
    ]);

    interval_set_a.difference(&interval_set_b);
}

