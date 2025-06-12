// Answer 0

#[test]
fn test_canonicalize_with_empty_ranges() {
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower: u32,
        upper: u32,
    }

    impl Debug for TestInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "TestInterval({}, {})", self.lower, self.upper)
        }
    }

    impl Interval for TestInterval {
        type Bound = u32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool { self.upper + 1 >= other.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![]);
    interval_set.canonicalize();
}

#[test]
#[should_panic]
fn test_canonicalize_with_non_empty_ranges() {
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestInterval {
        lower: u32,
        upper: u32,
    }

    impl Debug for TestInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "TestInterval({}, {})", self.lower, self.upper)
        }
    }

    impl Interval for TestInterval {
        type Bound = u32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool { self.upper + 1 >= other.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![
        TestInterval { lower: 3, upper: 5 },
        TestInterval { lower: 1, upper: 2 },
    ]);
    interval_set.canonicalize();
}

