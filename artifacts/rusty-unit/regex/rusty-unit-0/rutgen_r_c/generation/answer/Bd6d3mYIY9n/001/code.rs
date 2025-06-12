// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(u8);

impl TestBound {
    fn decrement(self) -> Self {
        TestBound(self.0.saturating_sub(1))
    }
    fn increment(self) -> Self {
        TestBound(self.0.saturating_add(1))
    }
}

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

    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper == other.lower || other.upper == self.lower
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || other.upper < self.lower
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_is_canonical_false_due_to_non_canonical_ordering() {
    let intervals = vec![
        TestInterval { lower: TestBound(1), upper: TestBound(1) },
        TestInterval { lower: TestBound(1), upper: TestBound(1) },
    ];

    let interval_set = IntervalSet { ranges: intervals };
    assert!(!interval_set.is_canonical());
}

#[test]
fn test_is_canonical_false_due_to_contiguous_intervals() {
    let intervals = vec![
        TestInterval { lower: TestBound(1), upper: TestBound(2) },
        TestInterval { lower: TestBound(2), upper: TestBound(3) },
    ];

    let interval_set = IntervalSet { ranges: intervals };
    assert!(!interval_set.is_canonical());
} 

#[test]
fn test_is_canonical_true_with_non_contiguous_intervals() {
    let intervals = vec![
        TestInterval { lower: TestBound(1), upper: TestBound(1) },
        TestInterval { lower: TestBound(3), upper: TestBound(3) },
    ];

    let interval_set = IntervalSet { ranges: intervals };
    assert!(interval_set.is_canonical());
}

