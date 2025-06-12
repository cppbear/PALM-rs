// Answer 0

#[test]
fn test_canonicalize_single_interval() {
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
        fn is_contiguous(&self, other: &Self) -> bool { self.upper + 1 == other.lower || other.upper + 1 == self.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set = IntervalSet::new(vec![SimpleInterval { lower: 1, upper: 1 }]);

    set.canonicalize();
}

#[test]
fn test_canonicalize_multiple_non_overlapping_intervals() {
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
        fn is_contiguous(&self, other: &Self) -> bool { self.upper + 1 == other.lower || other.upper + 1 == self.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set = IntervalSet::new(vec![SimpleInterval { lower: 1, upper: 1 }, 
                                         SimpleInterval { lower: 2, upper: 2 },
                                         SimpleInterval { lower: 3, upper: 3 }]);

    set.canonicalize();
}

#[test]
fn test_canonicalize_multiple_contiguous_intervals() {
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
        fn is_contiguous(&self, other: &Self) -> bool { self.upper + 1 == other.lower || other.upper + 1 == self.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set = IntervalSet::new(vec![SimpleInterval { lower: 1, upper: 2 }, 
                                         SimpleInterval { lower: 2, upper: 3 },
                                         SimpleInterval { lower: 3, upper: 4 }]);

    set.canonicalize();
}

#[test]
fn test_canonicalize_non_overlapping_intervals() {
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
        fn is_contiguous(&self, other: &Self) -> bool { self.upper + 1 == other.lower || other.upper + 1 == self.lower }
        fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
        fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
    }

    let mut set = IntervalSet::new(vec![SimpleInterval { lower: 1, upper: 10 },
                                         SimpleInterval { lower: 11, upper: 20 },
                                         SimpleInterval { lower: 21, upper: 30 }]);

    set.canonicalize();
}

