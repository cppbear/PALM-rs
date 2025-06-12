// Answer 0

#[test]
fn test_symmetric_difference_basic() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(u8);
    
    impl Bound {
        pub fn decrement(&self) -> Bound {
            Bound(self.0.saturating_sub(1))
        }

        pub fn increment(&self) -> Bound {
            Bound(self.0.saturating_add(1))
        }
    }
    
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct TestInterval {
        lower: Bound,
        upper: Bound,
    }
    
    impl Interval for TestInterval {
        type Bound = Bound;

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

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}

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

    let mut set_a = IntervalSet::new(vec![TestInterval { lower: Bound(1), upper: Bound(5) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: Bound(3), upper: Bound(7) }]);
    
    set_a.symmetric_difference(&set_b);
    
    let expected_result = vec![
        TestInterval { lower: Bound(1), upper: Bound(2) },
        TestInterval { lower: Bound(6), upper: Bound(7) },
    ];
    
    assert_eq!(set_a.intervals(), &expected_result);
}

#[test]
fn test_symmetric_difference_no_overlap() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(u8);
    
    impl Bound {
        pub fn decrement(&self) -> Bound {
            Bound(self.0.saturating_sub(1))
        }

        pub fn increment(&self) -> Bound {
            Bound(self.0.saturating_add(1))
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct TestInterval {
        lower: Bound,
        upper: Bound,
    }
    
    impl Interval for TestInterval {
        type Bound = Bound;

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

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}

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

    let mut set_a = IntervalSet::new(vec![TestInterval { lower: Bound(1), upper: Bound(2) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: Bound(3), upper: Bound(4) }]);
    
    set_a.symmetric_difference(&set_b);
    
    let expected_result = vec![
        TestInterval { lower: Bound(1), upper: Bound(2) },
        TestInterval { lower: Bound(3), upper: Bound(4) },
    ];
    
    assert_eq!(set_a.intervals(), &expected_result);
} 

#[test]
fn test_symmetric_difference_full_overlap() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(u8);
    
    impl Bound {
        pub fn decrement(&self) -> Bound {
            Bound(self.0.saturating_sub(1))
        }

        pub fn increment(&self) -> Bound {
            Bound(self.0.saturating_add(1))
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct TestInterval {
        lower: Bound,
        upper: Bound,
    }
    
    impl Interval for TestInterval {
        type Bound = Bound;

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

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}

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

    let mut set_a = IntervalSet::new(vec![TestInterval { lower: Bound(1), upper: Bound(4) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: Bound(1), upper: Bound(4) }]);
    
    set_a.symmetric_difference(&set_b);
    
    let expected_result: Vec<TestInterval> = vec![];
    
    assert_eq!(set_a.intervals(), &expected_result);
}

