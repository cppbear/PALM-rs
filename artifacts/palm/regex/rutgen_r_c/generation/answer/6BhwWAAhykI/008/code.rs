// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct SimpleBound(u32);

impl SimpleBound {
    fn increment(&self) -> SimpleBound {
        SimpleBound(self.0 + 1)
    }

    fn decrement(&self) -> SimpleBound {
        SimpleBound(self.0 - 1)
    }
}

impl Debug for SimpleBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

    fn case_fold_simple(&self, _: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper.0 >= other.lower.0
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper.0 < other.lower.0
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
    }
}

#[test]
fn test_intersect_with_non_empty_sets() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
        SimpleInterval { lower: SimpleBound(6), upper: SimpleBound(10) },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(4), upper: SimpleBound(8) },
        SimpleInterval { lower: SimpleBound(12), upper: SimpleBound(15) },
    ]);

    set_a.intersect(&set_b);

    assert_eq!(set_a.intervals(), &[
        SimpleInterval { lower: SimpleBound(4), upper: SimpleBound(5) },
        SimpleInterval { lower: SimpleBound(6), upper: SimpleBound(8) },
    ]);
}

#[test]
fn test_intersect_with_non_overlapping_sets() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(3) },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(4), upper: SimpleBound(6) },
    ]);

    set_a.intersect(&set_b);

    assert_eq!(set_a.intervals(), &[]);
}

#[test]
fn test_intersect_with_subsets() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(10) },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(2), upper: SimpleBound(5) },
    ]);

    set_a.intersect(&set_b);

    assert_eq!(set_a.intervals(), &[
        SimpleInterval { lower: SimpleBound(2), upper: SimpleBound(5) },
    ]);
}

#[test]
fn test_intersect_with_empty_result() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(2), upper: SimpleBound(3) },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(3), upper: SimpleBound(4) },
    ]);

    set_a.intersect(&set_b);

    assert_eq!(set_a.intervals(), &[]);
}

