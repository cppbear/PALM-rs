// Answer 0

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct SimpleBound(u32);
impl Bound for SimpleBound {}
impl SimpleBound {
    fn decrement(&self) -> Self {
        SimpleBound(self.0 - 1)
    }
    fn increment(&self) -> Self {
        SimpleBound(self.0 + 1)
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
        self.upper >= other.lower || self.lower <= other.upper
    }
    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }
    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_canonicalize_with_non_empty_ranges() {
    let mut interval_set = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(1),
            upper: SimpleBound(5),
        },
        SimpleInterval {
            lower: SimpleBound(10),
            upper: SimpleBound(15),
        },
        SimpleInterval {
            lower: SimpleBound(5),
            upper: SimpleBound(8),
        },
    ]);

    // The current ranges are not canonical, as the overlaps exist
    interval_set.canonicalize();

    // After canonicalization, ranges should be merged and sorted
    let expected_intervals = vec![
        SimpleInterval {
            lower: SimpleBound(1),
            upper: SimpleBound(8),
        },
        SimpleInterval {
            lower: SimpleBound(10),
            upper: SimpleBound(15),
        },
    ];
    assert_eq!(interval_set.intervals().to_vec(), expected_intervals);
}

#[test]
#[should_panic]
fn test_canonicalize_should_panic_empty_ranges() {
    let mut empty_interval_set: IntervalSet<SimpleInterval> = IntervalSet::new(vec![]);

    empty_interval_set.canonicalize();
}

#[test]
#[should_panic]
fn test_canonicalize_should_panic_on_drain() {
    let mut interval_set = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(1),
            upper: SimpleBound(3),
        },
        SimpleInterval {
            lower: SimpleBound(2),
            upper: SimpleBound(6),
        },
    ]);
    interval_set.canonicalize();
    // Here we manipulate the intervals to force a panic situation
    interval_set.ranges.push(SimpleInterval {
        lower: SimpleBound(4),
        upper: SimpleBound(2),
    });
    
    interval_set.canonicalize();
}

