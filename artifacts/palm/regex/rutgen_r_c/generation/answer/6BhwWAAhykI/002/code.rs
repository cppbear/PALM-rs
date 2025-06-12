// Answer 0

#[test]
fn test_intersect_with_non_empty_self_and_empty_other() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(u8);
    
    impl Bound {
        fn decrement(&self) -> Self {
            Bound(self.0.saturating_sub(1))
        }
        fn increment(&self) -> Self {
            Bound(self.0.saturating_add(1))
        }
    }

    impl Interval for Bound {
        type Bound = u8;

        fn lower(&self) -> Self::Bound {
            self.0
        }

        fn upper(&self) -> Self::Bound {
            self.0
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.0 = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.0 = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.0 != other.0
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut set_a = IntervalSet { ranges: vec![Bound(1), Bound(2), Bound(3)] };
    let set_b = IntervalSet { ranges: vec![] };

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 0);
}

#[test]
fn test_intersect_with_non_empty_self_and_empty_other_multiple_elements() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound(u8);
    
    impl Bound {
        fn decrement(&self) -> Self {
            Bound(self.0.saturating_sub(1))
        }
        fn increment(&self) -> Self {
            Bound(self.0.saturating_add(1))
        }
    }

    impl Interval for Bound {
        type Bound = u8;

        fn lower(&self) -> Self::Bound {
            self.0
        }

        fn upper(&self) -> Self::Bound {
            self.0
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.0 = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.0 = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.0 != other.0
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut set_a = IntervalSet { ranges: vec![Bound(1), Bound(2), Bound(3)] };
    let set_b = IntervalSet { ranges: vec![] };

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 0);
}

