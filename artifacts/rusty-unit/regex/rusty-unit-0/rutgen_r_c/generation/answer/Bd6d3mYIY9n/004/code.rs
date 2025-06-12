// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(i32);

impl TestBound {
    fn increment(&self) -> Self {
        TestBound(self.0 + 1)
    }
    
    fn decrement(&self) -> Self {
        TestBound(self.0 - 1)
    }
}

impl Bound for TestBound {}

#[derive(Clone, Debug, PartialEq)]
struct TestInterval {
    lower: TestBound,
    upper: TestBound,
}

impl Default for TestInterval {
    fn default() -> Self {
        Self {
            lower: TestBound(0),
            upper: TestBound(0),
        }
    }
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
        self.upper == other.lower || self.lower == other.upper
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_subset(&self, _other: &Self) -> bool {
        false
    }
}

#[test]
fn test_is_canonical_empty_ranges() {
    let interval_set = IntervalSet { ranges: vec![] };
    assert!(interval_set.is_canonical());
}

#[test]
fn test_is_canonical_single_range() {
    let interval = TestInterval {
        lower: TestBound(1),
        upper: TestBound(2),
    };
    let interval_set = IntervalSet { ranges: vec![interval] };
    assert!(interval_set.is_canonical());
}

#[test]
fn test_is_canonical_non_contiguous_ranges() {
    let interval1 = TestInterval {
        lower: TestBound(1),
        upper: TestBound(2),
    };
    let interval2 = TestInterval {
        lower: TestBound(3),
        upper: TestBound(4),
    };
    let interval_set = IntervalSet { ranges: vec![interval1, interval2] };
    assert!(interval_set.is_canonical());
}

#[test]
fn test_is_canonical_invalid_ordering() {
    let interval1 = TestInterval {
        lower: TestBound(2),
        upper: TestBound(3),
    };
    let interval2 = TestInterval {
        lower: TestBound(1),
        upper: TestBound(2),
    };
    let interval_set = IntervalSet { ranges: vec![interval1, interval2] };
    assert!(!interval_set.is_canonical());
}

#[test]
fn test_is_canonical_contiguous_ranges() {
    let interval1 = TestInterval {
        lower: TestBound(1),
        upper: TestBound(2),
    };
    let interval2 = TestInterval {
        lower: TestBound(2),
        upper: TestBound(3),
    };
    let interval_set = IntervalSet { ranges: vec![interval1, interval2] };
    assert!(!interval_set.is_canonical());
}

