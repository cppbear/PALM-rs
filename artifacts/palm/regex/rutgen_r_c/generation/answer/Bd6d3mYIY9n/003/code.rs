// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(u32);

impl Bound for TestBound {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestInterval {
    lower: TestBound,
    upper: TestBound,
}

impl Interval for TestInterval {
    type Bound = TestBound;

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
        self.lower > other.upper || self.upper < other.lower
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_is_canonical_true() {
    let intervals = vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(2),
        },
        TestInterval {
            lower: TestBound(3),
            upper: TestBound(4),
        },
        TestInterval {
            lower: TestBound(5),
            upper: TestBound(6),
        },
    ];
    let interval_set = IntervalSet::new(intervals);
    assert!(interval_set.is_canonical());
}

#[test]
fn test_is_canonical_pair_greater_or_equal() {
    let intervals = vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(3),
        },
        TestInterval {
            lower: TestBound(2),
            upper: TestBound(4),
        },
    ];
    let interval_set = IntervalSet::new(intervals);
    assert!(!interval_set.is_canonical());
}

#[test]
fn test_is_canonical_pair_contiguous() {
    let intervals = vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(2),
        },
        TestInterval {
            lower: TestBound(2),
            upper: TestBound(3),
        },
    ];
    let interval_set = IntervalSet::new(intervals);
    assert!(!interval_set.is_canonical());
}

#[test]
fn test_is_canonical_empty() {
    let intervals: Vec<TestInterval> = vec![];
    let interval_set = IntervalSet::new(intervals);
    assert!(interval_set.is_canonical());
}

