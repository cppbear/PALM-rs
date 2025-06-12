// Answer 0

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleBound {
    value: i32,
}

impl SimpleBound {
    pub fn new(value: i32) -> Self {
        SimpleBound { value }
    }
}

impl std::ops::Deref for SimpleBound {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::cmp::PartialOrd for SimpleBound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl std::cmp::Ord for SimpleBound {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl std::fmt::Debug for SimpleBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpleBound({})", self.value)
    }
}

pub struct SimpleInterval {
    lower: SimpleBound,
    upper: SimpleBound,
}

impl Default for SimpleInterval {
    fn default() -> Self {
        SimpleInterval {
            lower: SimpleBound::new(0),
            upper: SimpleBound::new(0),
        }
    }
}

impl Interval for SimpleInterval {
    type Bound = SimpleBound;

    fn lower(&self) -> Self::Bound {
        self.lower.clone()
    }

    fn upper(&self) -> Self::Bound {
        self.upper.clone()
    }

    fn set_lower(&mut self, bound: Self::Bound) {
        self.lower = bound;
    }

    fn set_upper(&mut self, bound: Self::Bound) {
        self.upper = bound;
    }

    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_canonicalize_with_non_canonical_intervals() {
    let mut set = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound::new(1),
            upper: SimpleBound::new(5),
        },
        SimpleInterval {
            lower: SimpleBound::new(3),
            upper: SimpleBound::new(7),
        },
        SimpleInterval {
            lower: SimpleBound::new(10),
            upper: SimpleBound::new(15),
        },
    ]);

    // Ensure the set is not canonical before calling canonicalize
    set.canonicalize();
    
    assert_eq!(set.intervals(), &[
        SimpleInterval {
            lower: SimpleBound::new(1),
            upper: SimpleBound::new(7),
        },
        SimpleInterval {
            lower: SimpleBound::new(10),
            upper: SimpleBound::new(15),
        },
    ]);
}

#[test]
#[should_panic]
fn test_canonicalize_with_empty_ranges() {
    let mut set = IntervalSet::new(vec![]);
    set.canonicalize();
}

#[test]
fn test_canonicalize_panic_conditions() {
    let mut set = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound::new(1),
            upper: SimpleBound::new(4),
        },
        SimpleInterval {
            lower: SimpleBound::new(2),
            upper: SimpleBound::new(3),
        },
        SimpleInterval {
            lower: SimpleBound::new(3),
            upper: SimpleBound::new(6),
        },
    ]);

    // Run the canonicalize function which should not panic and produce a valid set
    set.canonicalize();
    
    assert_eq!(set.intervals(), &[
        SimpleInterval {
            lower: SimpleBound::new(1),
            upper: SimpleBound::new(6),
        },
    ]);
}

