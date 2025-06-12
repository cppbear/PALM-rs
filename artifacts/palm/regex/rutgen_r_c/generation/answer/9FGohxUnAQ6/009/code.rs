// Answer 0

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
struct SimpleBound(u32);

impl SimpleBound {
    fn increment(self) -> Self {
        SimpleBound(self.0 + 1)
    }

    fn decrement(self) -> Self {
        SimpleBound(self.0 - 1)
    }
}

impl std::fmt::Debug for SimpleBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for SimpleBound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for SimpleBound {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Eq for SimpleBound {}

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
        self.upper.0 >= other.lower.0 && self.lower.0 <= other.upper.0
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
    }
}

#[test]
fn test_difference_non_overlapping_ranges() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
        SimpleInterval { lower: SimpleBound(10), upper: SimpleBound(15) },
    ]);
    let set_b = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(6), upper: SimpleBound(9) },
    ]);

    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), &[
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
        SimpleInterval { lower: SimpleBound(10), upper: SimpleBound(15) },
    ]);
}

#[test]
fn test_difference_with_complete_overlap() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
    ]);
    let set_b = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
    ]);

    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), &[]);
}

#[test]
fn test_difference_partial_overlap() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(3), upper: SimpleBound(8) },
    ]);
    let set_b = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(5), upper: SimpleBound(10) },
    ]);

    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), &[SimpleInterval { lower: SimpleBound(3), upper: SimpleBound(5) }]);
}

#[test]
fn test_difference_adjacent() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
        SimpleInterval { lower: SimpleBound(6), upper: SimpleBound(10) },
    ]);
    let set_b = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(5), upper: SimpleBound(6) },
    ]);

    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), &[
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
        SimpleInterval { lower: SimpleBound(6), upper: SimpleBound(10) },
    ]);
}

#[test]
fn test_difference_empty_set_b() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
    ]);
    let set_b = IntervalSet::new(vec![]);

    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), &[
        SimpleInterval { lower: SimpleBound(1), upper: SimpleBound(5) },
    ]);
}

