// Answer 0

fn test_canonicalize_non_canonical_with_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(usize);
    
    impl Bound {
        fn decrement(&self) -> Self {
            Bound(self.0.saturating_sub(1))
        }
        
        fn increment(&self) -> Self {
            Bound(self.0 + 1)
        }
    }

    impl Bound {
        fn is_contiguous(&self, other: &Self) -> bool {
            (self.0 + 1) == other.0 || (other.0 + 1) == self.0
        }
    }

    impl PartialOrd for Bound {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Bound {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl Eq for Bound {}

    impl Interval for Bound {
        type Bound = Self;

        fn lower(&self) -> Self::Bound {
            *self
        }

        fn upper(&self) -> Self::Bound {
            *self
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.is_contiguous(other)
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.0 >= other.0
        }
    }

    let mut interval_set = IntervalSet {
        ranges: vec![Bound(1), Bound(3), Bound(2)],
    };

    interval_set.canonicalize();

    assert_eq!(interval_set.ranges, vec![Bound(1), Bound(2), Bound(3)]);
}

fn test_canonicalize_empty() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(usize);

    impl Bound {
        fn decrement(&self) -> Self {
            Bound(self.0.saturating_sub(1))
        }

        fn increment(&self) -> Self {
            Bound(self.0 + 1)
        }
    }

    impl PartialOrd for Bound {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Bound {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl Eq for Bound {}

    impl Interval for Bound {
        type Bound = Self;

        fn lower(&self) -> Self::Bound {
            *self
        }

        fn upper(&self) -> Self::Bound {
            *self
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.0 + 1 == other.0
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut interval_set = IntervalSet {
        ranges: vec![],
    };

    let result = std::panic::catch_unwind(|| {
        interval_set.canonicalize();
    });

    assert!(result.is_err());
}

fn test_canonicalize_with_consecutive_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(usize);

    impl Bound {
        fn decrement(&self) -> Self {
            Bound(self.0.saturating_sub(1))
        }

        fn increment(&self) -> Self {
            Bound(self.0 + 1)
        }
    }

    impl PartialOrd for Bound {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Bound {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl Eq for Bound {}

    impl Interval for Bound {
        type Bound = Self;

        fn lower(&self) -> Self::Bound {
            *self
        }

        fn upper(&self) -> Self::Bound {
            *self
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            *self = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.0 + 1 == other.0
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }
    
    let mut interval_set = IntervalSet {
        ranges: vec![Bound(1), Bound(2), Bound(4), Bound(3)],
    };

    interval_set.canonicalize();

    assert_eq!(interval_set.ranges, vec![Bound(1), Bound(2), Bound(3), Bound(4)]);
}

