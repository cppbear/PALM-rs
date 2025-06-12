// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct SimpleBound(i32);

impl std::fmt::Display for SimpleBound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SimpleBound {
    fn decrement(&self) -> SimpleBound {
        SimpleBound(self.0 - 1)
    }

    fn increment(&self) -> SimpleBound {
        SimpleBound(self.0 + 1)
    }
}

impl Interval for SimpleBound {
    type Bound = Self;

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

    fn case_fold_simple(&self, _: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        (self.0 + 1) == other.0 || (self.0 - 1) == other.0
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper().0 < other.lower().0 || self.lower().0 > other.upper().0
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower().0 >= other.lower().0 && self.upper().0 <= other.upper().0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IntervalSetTest {
    ranges: Vec<SimpleBound>,
}

impl IntervalSet<SimpleBound> {
    pub fn new(intervals: Vec<SimpleBound>) -> Self {
        IntervalSet { ranges: intervals }
    }
}

#[test]
fn test_canonicalize_non_canonical() {
    let mut set = IntervalSet::new(vec![SimpleBound(1), SimpleBound(2), SimpleBound(4), SimpleBound(6)]);
    set.ranges.push(SimpleBound(5)); // Adding an overlapping range
    set.canonicalize();
}

#[test]
fn test_canonicalize_with_empty() {
    let mut set = IntervalSet::new(vec![SimpleBound(1), SimpleBound(3)]);
    set.ranges.push(SimpleBound(2)); // This creates the need for canonicalization
    set.canonicalize();
}

#[test]
fn test_canonicalize_multiple_merges() {
    let mut set = IntervalSet::new(vec![SimpleBound(1), SimpleBound(4), SimpleBound(5)]);
    set.ranges.push(SimpleBound(2)); // Should merge into 1..5
    set.canonicalize();
}

#[test]
fn test_canonicalize_non_merge() {
    let mut set = IntervalSet::new(vec![SimpleBound(1), SimpleBound(3)]);
    set.ranges.push(SimpleBound(6)); // Separate range; no merging should happen
    set.canonicalize();
}

#[test]
fn test_canonicalize_edge_case() {
    let mut set = IntervalSet::new(vec![SimpleBound(3)]);
    set.ranges.push(SimpleBound(2)); // Should create a merge: 2..3
    set.canonicalize();
}

