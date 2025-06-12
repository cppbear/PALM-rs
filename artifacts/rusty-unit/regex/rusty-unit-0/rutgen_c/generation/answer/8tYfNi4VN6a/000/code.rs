// Answer 0

#[test]
fn test_canonicalize_empty_set() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper.0 + 1 == other.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set = IntervalSet::<TestInterval>::new(vec![]);
    set.canonicalize();
}

#[test]
fn test_canonicalize_single_interval() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper.0 + 1 == other.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set = IntervalSet::<TestInterval>::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    set.canonicalize();
    assert_eq!(set.intervals().len(), 1);
    assert_eq!(set.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(5) });
}

#[test]
fn test_canonicalize_overlapping_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper.0 + 1 == other.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set = IntervalSet::<TestInterval>::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
        TestInterval { lower: TestBound(4), upper: TestBound(8) },
    ]);
    set.canonicalize();
    assert_eq!(set.intervals().len(), 1);
    assert_eq!(set.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(8) });
}

#[test]
fn test_canonicalize_non_overlapping_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);
    
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper.0 + 1 == other.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut set = IntervalSet::<TestInterval>::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(2) },
        TestInterval { lower: TestBound(3), upper: TestBound(4) },
    ]);
    set.canonicalize();
    assert_eq!(set.intervals().len(), 2);
    assert_eq!(set.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(2) });
    assert_eq!(set.intervals()[1], TestInterval { lower: TestBound(3), upper: TestBound(4) });
}

