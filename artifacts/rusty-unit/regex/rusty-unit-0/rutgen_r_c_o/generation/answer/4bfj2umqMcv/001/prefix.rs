// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct SimpleBound(u8);

impl SimpleBound {
    fn decrement(&self) -> Self {
        Self(self.0.saturating_sub(1))
    }

    fn increment(&self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

impl PartialOrd for SimpleBound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for SimpleBound {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
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
        self.upper >= other.lower && self.lower <= other.upper
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_union_with_non_empty_intervals() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(5),
            upper: SimpleBound(10),
        },
        SimpleInterval {
            lower: SimpleBound(15),
            upper: SimpleBound(20),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(10),
            upper: SimpleBound(30),
        },
    ]);

    set_a.union(&set_b);
}

#[test]
fn test_union_with_overlapping_intervals() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(0),
            upper: SimpleBound(10),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(5),
            upper: SimpleBound(15),
        },
    ]);

    set_a.union(&set_b);
}

#[test]
fn test_union_with_non_contiguous_intervals() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(0),
            upper: SimpleBound(5),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(10),
            upper: SimpleBound(15),
        },
    ]);

    set_a.union(&set_b);
}

#[test]
fn test_union_with_identical_intervals() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(0),
            upper: SimpleBound(5),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(0),
            upper: SimpleBound(5),
        },
    ]);

    set_a.union(&set_b);
}

#[test]
fn test_union_with_single_element_set() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(1),
            upper: SimpleBound(2),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(2),
            upper: SimpleBound(3),
        },
    ]);

    set_a.union(&set_b);
}

#[test]
fn test_union_large_intervals() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(0),
            upper: SimpleBound(255),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(128),
            upper: SimpleBound(200),
        },
    ]);

    set_a.union(&set_b);
}

