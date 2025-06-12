// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct SimpleBound(u32);

impl Bound for SimpleBound {
    fn increment(self) -> Self {
        SimpleBound(self.0 + 1)
    }
    
    fn decrement(self) -> Self {
        SimpleBound(self.0 - 1)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct SimpleInterval {
    lower_bound: SimpleBound,
    upper_bound: SimpleBound,
}

impl Interval for SimpleInterval {
    type Bound = SimpleBound;

    fn lower(&self) -> Self::Bound {
        self.lower_bound
    }

    fn upper(&self) -> Self::Bound {
        self.upper_bound
    }

    fn set_lower(&mut self, bound: Self::Bound) {
        self.lower_bound = bound;
    }

    fn set_upper(&mut self, bound: Self::Bound) {
        self.upper_bound = bound;
    }

    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        self.lower() <= other.upper() && other.lower() <= self.upper()
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper() < other.lower() || self.lower() > other.upper()
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower() >= other.lower() && self.upper() <= other.upper()
    }
}

#[test]
fn test_difference_non_empty_intervals() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower_bound: SimpleBound(1),
            upper_bound: SimpleBound(5),
        },
        SimpleInterval {
            lower_bound: SimpleBound(7),
            upper_bound: SimpleBound(10),
        },
    ]);
    
    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower_bound: SimpleBound(3),
            upper_bound: SimpleBound(8),
        },
    ]);
    
    set_a.difference(&set_b);
    
    let expected = vec![
        SimpleInterval {
            lower_bound: SimpleBound(1),
            upper_bound: SimpleBound(2),
        },
        SimpleInterval {
            lower_bound: SimpleBound(8),
            upper_bound: SimpleBound(10),
        },
    ];

    assert_eq!(set_a.intervals(), &expected);
}

#[test]
fn test_difference_disjoint_intervals() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower_bound: SimpleBound(1),
            upper_bound: SimpleBound(2),
        },
        SimpleInterval {
            lower_bound: SimpleBound(5),
            upper_bound: SimpleBound(7),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower_bound: SimpleBound(10),
            upper_bound: SimpleBound(12),
        },
    ]);
    
    set_a.difference(&set_b);
    
    let expected = vec![
        SimpleInterval {
            lower_bound: SimpleBound(1),
            upper_bound: SimpleBound(2),
        },
        SimpleInterval {
            lower_bound: SimpleBound(5),
            upper_bound: SimpleBound(7),
        },
    ];

    assert_eq!(set_a.intervals(), &expected);
}

#[test]
fn test_difference_all_covered() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower_bound: SimpleBound(1),
            upper_bound: SimpleBound(10),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower_bound: SimpleBound(1),
            upper_bound: SimpleBound(10),
        },
    ]);

    set_a.difference(&set_b);

    let expected: Vec<SimpleInterval> = Vec::new();

    assert_eq!(set_a.intervals(), &expected);
}

#[test]
fn test_difference_with_empty_first() {
    let mut set_a = IntervalSet::new(vec![]);
    let set_b = IntervalSet::new(vec![
        SimpleInterval {
            lower_bound: SimpleBound(2),
            upper_bound: SimpleBound(5),
        },
    ]);

    set_a.difference(&set_b);
    
    let expected: Vec<SimpleInterval> = Vec::new();

    assert_eq!(set_a.intervals(), &expected);
}

#[test]
fn test_difference_with_empty_second() {
    let mut set_a = IntervalSet::new(vec![
        SimpleInterval {
            lower_bound: SimpleBound(1),
            upper_bound: SimpleBound(10),
        },
    ]);
    let set_b = IntervalSet::new(vec![]);

    set_a.difference(&set_b);
    
    let expected = vec![
        SimpleInterval {
            lower_bound: SimpleBound(1),
            upper_bound: SimpleBound(10),
        },
    ];

    assert_eq!(set_a.intervals(), &expected);
}

