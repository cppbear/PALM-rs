// Answer 0

#[test]
fn test_difference_non_overlapping_ranges() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct SimpleInterval {
        lower: u32,
        upper: u32,
    }

    impl Interval for SimpleInterval {
        type Bound = u32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool { self.lower <= other.upper && self.upper >= other.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set_a = IntervalSet::new(vec![SimpleInterval { lower: 1, upper: 5 }]);
    let set_b = IntervalSet::new(vec![SimpleInterval { lower: 5, upper: 10 }]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_with_partial_overlap() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct SimpleInterval {
        lower: u32,
        upper: u32,
    }

    impl Interval for SimpleInterval {
        type Bound = u32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool { self.lower <= other.upper && self.upper >= other.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set_a = IntervalSet::new(vec![SimpleInterval { lower: 1, upper: 10 }]);
    let set_b = IntervalSet::new(vec![SimpleInterval { lower: 5, upper: 15 }]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_full_overlap() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct SimpleInterval {
        lower: u32,
        upper: u32,
    }

    impl Interval for SimpleInterval {
        type Bound = u32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool { self.lower <= other.upper && self.upper >= other.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set_a = IntervalSet::new(vec![SimpleInterval { lower: 1, upper: 5 }]);
    let set_b = IntervalSet::new(vec![SimpleInterval { lower: 1, upper: 5 }]);

    set_a.difference(&set_b);
}

#[test]
fn test_difference_with_no_overlap() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct SimpleInterval {
        lower: u32,
        upper: u32,
    }

    impl Interval for SimpleInterval {
        type Bound = u32;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, other: &Self) -> bool { self.lower <= other.upper && self.upper >= other.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set_a = IntervalSet::new(vec![SimpleInterval { lower: 10, upper: 15 }]);
    let set_b = IntervalSet::new(vec![SimpleInterval { lower: 1, upper: 5 }]);

    set_a.difference(&set_b);
}

