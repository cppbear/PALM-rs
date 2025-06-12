// Answer 0

#[test]
fn test_difference_empty_self() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(u8);
    
    impl Bound {
        fn increment(self) -> Self {
            Bound(self.0 + 1)
        }
        fn decrement(self) -> Self {
            Bound(self.0.saturating_sub(1))
        }
    }

    impl Debug for Bound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct SimpleInterval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval for SimpleInterval {
        type Bound = Bound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }

        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut empty_set = IntervalSet::<SimpleInterval> { ranges: vec![] };
    let other_set = IntervalSet { ranges: vec![SimpleInterval { lower: Bound(1), upper: Bound(5) }] };

    empty_set.difference(&other_set);

    assert_eq!(empty_set.ranges.len(), 0);
}

#[test]
fn test_difference_empty_other() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(u8);
    
    impl Bound {
        fn increment(self) -> Self {
            Bound(self.0 + 1)
        }
        fn decrement(self) -> Self {
            Bound(self.0.saturating_sub(1))
        }
    }

    impl Debug for Bound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct SimpleInterval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval for SimpleInterval {
        type Bound = Bound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }

        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut set_with_intervals = IntervalSet::<SimpleInterval> { ranges: vec![SimpleInterval { lower: Bound(2), upper: Bound(4) }] };
    let empty_set = IntervalSet::<SimpleInterval> { ranges: vec![] };

    set_with_intervals.difference(&empty_set);
    
    assert_eq!(set_with_intervals.ranges.len(), 1);
    assert_eq!(set_with_intervals.ranges[0], SimpleInterval { lower: Bound(2), upper: Bound(4) });
}

