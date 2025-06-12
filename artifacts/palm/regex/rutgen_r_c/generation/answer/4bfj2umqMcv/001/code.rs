// Answer 0

#[test]
fn test_union_non_empty_sets() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound(u32);
    
    impl Bound for TestBound {
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set1 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(5), upper: TestBound(7) },
    ]);

    let set2 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(2), upper: TestBound(4) },
        TestInterval { lower: TestBound(6), upper: TestBound(8) },
    ]);

    set1.union(&set2);
    
    assert_eq!(set1.intervals(), &vec![
        TestInterval { lower: TestBound(1), upper: TestBound(4) },
        TestInterval { lower: TestBound(5), upper: TestBound(8) },
    ]);
}

#[test]
fn test_union_with_empty_set() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound(u32);
    
    impl Bound for TestBound {
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set1 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
    ]);

    let set2 = IntervalSet::new(Vec::<TestInterval>::new());

    set1.union(&set2);
    
    assert_eq!(set1.intervals(), &vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
    ]);
}

#[test]
fn test_union_non_contiguous_sets() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct TestBound(u32);
    
    impl Bound for TestBound {
        fn increment(&self) -> Self {
            TestBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            TestBound(self.0 - 1)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower
        }
        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }
        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set1 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
    ]);

    let set2 = IntervalSet::new(vec![
        TestInterval { lower: TestBound(4), upper: TestBound(6) },
    ]);

    let result = set1.union(&set2);
    
    assert_eq!(set1.intervals(), &vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(4), upper: TestBound(6) },
    ]);
}

