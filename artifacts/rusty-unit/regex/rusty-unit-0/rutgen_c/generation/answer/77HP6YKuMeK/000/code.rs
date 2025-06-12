// Answer 0

#[test]
fn test_intervals_empty() {
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound(u8);

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower.clone()
        }

        fn upper(&self) -> Self::Bound {
            self.upper.clone()
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool { true }

        fn is_intersection_empty(&self, _other: &Self) -> bool { false }

        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let empty_set: IntervalSet<TestInterval> = IntervalSet::new(vec![]);
    assert_eq!(empty_set.intervals(), &[]);
}

#[test]
fn test_intervals_single() {
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound(u8);

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower.clone()
        }

        fn upper(&self) -> Self::Bound {
            self.upper.clone()
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool { true }

        fn is_intersection_empty(&self, _other: &Self) -> bool { false }

        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let single_interval = TestInterval { lower: TestBound(1), upper: TestBound(5) };
    let set = IntervalSet::new(vec![single_interval.clone()]);
    assert_eq!(set.intervals(), &[single_interval]);
}

#[test]
fn test_intervals_multiple() {
    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestBound(u8);

    #[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct TestInterval {
        lower: TestBound,
        upper: TestBound,
    }

    impl Interval for TestInterval {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound {
            self.lower.clone()
        }

        fn upper(&self) -> Self::Bound {
            self.upper.clone()
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool { true }

        fn is_intersection_empty(&self, _other: &Self) -> bool { false }

        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let interval1 = TestInterval { lower: TestBound(1), upper: TestBound(3) };
    let interval2 = TestInterval { lower: TestBound(4), upper: TestBound(6) };
    let set = IntervalSet::new(vec![interval1.clone(), interval2.clone()]);
    assert_eq!(set.intervals(), &[interval1, interval2]);
}

