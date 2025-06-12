// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct SimpleBound(u32);

impl Bound for SimpleBound {
    fn increment(&self) -> Self {
        SimpleBound(self.0 + 1)
    }

    fn decrement(&self) -> Self {
        SimpleBound(self.0 - 1)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

    fn case_fold_simple(&self, _: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper >= other.lower
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_union_empty_sets() {
    let mut set1 = IntervalSet::<SimpleInterval>::new(vec![]);
    let set2 = IntervalSet::<SimpleInterval>::new(vec![]);
    set1.union(&set2);
    assert_eq!(set1.intervals().len(), 0);
}

#[test]
fn test_union_non_empty_set_with_empty_set() {
    let mut set1 = IntervalSet::<SimpleInterval>::new(vec![SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) }]);
    let set2 = IntervalSet::<SimpleInterval>::new(vec![]);
    set1.union(&set2);
    assert_eq!(set1.intervals().len(), 1);
}

#[test]
fn test_union_with_non_contiguous_set() {
    let mut set1 = IntervalSet::<SimpleInterval>::new(vec![SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) }]);
    let set2 = IntervalSet::<SimpleInterval>::new(vec![SimpleInterval { lower: SimpleBound(6), upper: SimpleBound(10) }]);
    set1.union(&set2);
    assert_eq!(set1.intervals().len(), 2); // expect 2 separate intervals
}

#[test]
fn test_union_with_contiguous_set() {
    let mut set1 = IntervalSet::<SimpleInterval>::new(vec![SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) }]);
    let set2 = IntervalSet::<SimpleInterval>::new(vec![SimpleInterval { lower: SimpleBound(5), upper: SimpleBound(10) }]);
    set1.union(&set2);
    assert_eq!(set1.intervals().len(), 1); // expect merged interval
    assert_eq!(set1.intervals()[0], SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(10) });
}

