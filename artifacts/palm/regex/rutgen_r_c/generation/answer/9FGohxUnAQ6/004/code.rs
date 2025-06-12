// Answer 0

#[test]
fn test_difference_non_overlapping_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl TestBound {
        fn decrement(self) -> Self {
            TestBound(self.0 - 1)
        }
        
        fn increment(self) -> Self {
            TestBound(self.0 + 1)
        }
    }
    
    impl Bound for TestBound {}
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;
        
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: TestBound(6), upper: TestBound(10) }]);
    
    set_a.difference(&set_b);
    
    assert_eq!(set_a.intervals(), &[TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
}

#[test]
fn test_difference_with_overlapping_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl TestBound {
        fn decrement(self) -> Self {
            TestBound(self.0 - 1)
        }
        
        fn increment(self) -> Self {
            TestBound(self.0 + 1)
        }
    }
    
    impl Bound for TestBound {}
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;
        
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(10) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: TestBound(5), upper: TestBound(7) }]);
    
    set_a.difference(&set_b);
    
    assert_eq!(set_a.intervals(), &[TestInterval { lower: TestBound(1), upper: TestBound(4) }, TestInterval { lower: TestBound(8), upper: TestBound(10) }]);
}

#[test]
fn test_difference_empty_self() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl TestBound {
        fn decrement(self) -> Self {
            TestBound(self.0 - 1)
        }
        
        fn increment(self) -> Self {
            TestBound(self.0 + 1)
        }
    }
    
    impl Bound for TestBound {}
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;
        
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    
    set_a.difference(&set_b);
    
    assert_eq!(set_a.intervals(), &[]);
} 

#[test]
fn test_difference_empty_other() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    impl TestBound {
        fn decrement(self) -> Self {
            TestBound(self.0 - 1)
        }
        
        fn increment(self) -> Self {
            TestBound(self.0 + 1)
        }
    }
    
    impl Bound for TestBound {}
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;
        
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let mut set_a = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    let set_b = IntervalSet::new(vec![]);
    
    set_a.difference(&set_b);
    
    assert_eq!(set_a.intervals(), &[TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
}

