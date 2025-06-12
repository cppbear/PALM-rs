// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct SimpleBound(u32);

impl SimpleBound {
    fn increment(self) -> Self {
        SimpleBound(self.0 + 1)
    }
    fn decrement(self) -> Self {
        SimpleBound(self.0 - 1)
    }
}

impl Bound for SimpleBound {}

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

    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        (self.upper == other.lower || self.lower == other.upper)
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_is_canonical_false_non_contiguous() {
    let mut iset = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(0), upper: SimpleBound(1) },
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(3) },
    ]);
    assert!(!iset.is_canonical());
}

#[test]
fn test_is_canonical_false_greater_than() {
    let mut iset = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(3) },
        SimpleInterval { lower: SimpleBound(0), upper: SimpleBound(2) },
    ]);
    assert!(!iset.is_canonical());
}

#[test]
fn test_is_canonical_false_contiguous() {
    let mut iset = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(0), upper: SimpleBound(2) },
        SimpleInterval { lower: SimpleBound(2), upper: SimpleBound(4) },
    ]);
    assert!(!iset.is_canonical());
}

