// Answer 0

#[test]
fn test_canonicalize_already_canonical() {
    #[derive(Clone, Debug, Eq, PartialEq, Default, Copy, Ord, PartialOrd)]
    struct SimpleBound(u32);
    
    impl Debug for SimpleBound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Bound({})", self.0)
        }
    }
    
    impl Bound for SimpleBound {}

    #[derive(Clone, Debug, Eq, PartialEq, Default)]
    struct SimpleInterval {
        lower: SimpleBound,
        upper: SimpleBound,
    }

    impl Interval for SimpleInterval {
        type Bound = SimpleBound;

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
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.lower > other.upper || self.upper < other.lower
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let intervals = vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(2) },
        SimpleInterval { lower: SimpleBound(3), upper: SimpleBound(4) },
    ];

    let mut interval_set = IntervalSet::new(intervals);
    
    interval_set.canonicalize(); // should not panic
}

#[test]
fn test_canonicalize_non_canonical() {
    #[derive(Clone, Debug, Eq, PartialEq, Default, Copy, Ord, PartialOrd)]
    struct SimpleBound(u32);
    
    impl Debug for SimpleBound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Bound({})", self.0)
        }
    }
    
    impl Bound for SimpleBound {}

    #[derive(Clone, Debug, Eq, PartialEq, Default)]
    struct SimpleInterval {
        lower: SimpleBound,
        upper: SimpleBound,
    }

    impl Interval for SimpleInterval {
        type Bound = SimpleBound;

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
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.lower > other.upper || self.upper < other.lower
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let intervals = vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
        SimpleInterval { lower: SimpleBound(3), upper: SimpleBound(7) },
    ];

    let mut interval_set = IntervalSet::new(intervals);
    
    interval_set.canonicalize(); // should not panic, and will create a canonical order
}

#[test]
#[should_panic]
fn test_canonicalize_empty_ranges() {
    #[derive(Clone, Debug, Eq, PartialEq, Default, Copy, Ord, PartialOrd)]
    struct SimpleBound(u32);
    
    impl Debug for SimpleBound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Bound({})", self.0)
        }
    }
    
    impl Bound for SimpleBound {}

    #[derive(Clone, Debug, Eq, PartialEq, Default)]
    struct SimpleInterval {
        lower: SimpleBound,
        upper: SimpleBound,
    }

    impl Interval for SimpleInterval {
        type Bound = SimpleBound;

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
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.lower > other.upper || self.upper < other.lower
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let intervals: Vec<SimpleInterval> = vec![];

    let mut interval_set = IntervalSet::new(intervals);
    
    interval_set.canonicalize(); // should panic due to empty ranges
}

