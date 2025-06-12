// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct Bound(u32);

impl Bound {
    fn lower(&self) -> u32 {
        self.0
    }

    fn upper(&self) -> u32 {
        self.0
    }

    fn decrement(&self) -> Bound {
        Bound(self.0 - 1)
    }

    fn increment(&self) -> Bound {
        Bound(self.0 + 1)
    }
}

impl Bound {}

impl Interval for Bound {
    type Bound = Bound;
    
    fn lower(&self) -> Self::Bound {
        *self
    }

    fn upper(&self) -> Self::Bound {
        *self
    }

    fn set_lower(&mut self, bound: Self::Bound) {
        self.0 = bound.0;
    }

    fn set_upper(&mut self, bound: Self::Bound) {
        self.0 = bound.0;
    }

    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

    fn is_contiguous(&self, _other: &Self) -> bool {
        true
    }

    fn is_intersection_empty(&self, _other: &Self) -> bool {
        false
    }

    fn is_subset(&self, _other: &Self) -> bool {
        false
    }
}

#[test]
fn test_difference_non_empty_self_empty_other() {
    let mut interval_set1 = IntervalSet::<Bound>::new(vec![Bound(5), Bound(10)]);
    let interval_set2 = IntervalSet::<Bound>::new(vec![]);  // other is empty

    interval_set1.difference(&interval_set2);

    assert_eq!(interval_set1.intervals(), &[Bound(5), Bound(10)]);
}

#[test]
fn test_difference_multiple_ranges_self_empty_other() {
    let mut interval_set1 = IntervalSet::<Bound>::new(vec![Bound(1), Bound(3), Bound(5)]);
    let interval_set2 = IntervalSet::<Bound>::new(vec![]);  // other is empty

    interval_set1.difference(&interval_set2);

    assert_eq!(interval_set1.intervals(), &[Bound(1), Bound(3), Bound(5)]);
}

