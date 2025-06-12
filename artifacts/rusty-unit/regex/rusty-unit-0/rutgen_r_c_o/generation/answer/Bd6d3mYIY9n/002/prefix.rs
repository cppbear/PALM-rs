// Answer 0

#[derive(Clone, Copy, Debug, PartialEq)]
struct TestBound(u32);

impl std::cmp::PartialOrd for TestBound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl std::cmp::Ord for TestBound {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::fmt::Debug for TestBound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TestBound({})", self.0)
    }
}

impl Default for TestBound {
    fn default() -> Self {
        TestBound(0)
    }
}

#[derive(Clone, Debug, PartialEq)]
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

    fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper == other.lower
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        !(self.lower <= other.upper && self.upper >= other.lower)
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_is_canonical_with_non_contiguous_ranges() {
    let intervals = vec![
        TestInterval { lower: TestBound(1), upper: TestBound(2) },
        TestInterval { lower: TestBound(2), upper: TestBound(3) },
        TestInterval { lower: TestBound(3), upper: TestBound(4) },
        TestInterval { lower: TestBound(4), upper: TestBound(5) }, // This is non-contiguous
    ];

    let interval_set = IntervalSet::new(intervals);
    interval_set.is_canonical(); // This should evaluate to false
}

#[test]
fn test_is_canonical_with_overlapping_intervals() {
    let intervals = vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(2), upper: TestBound(4) }, // Overlaps
        TestInterval { lower: TestBound(5), upper: TestBound(6) },
    ];

    let interval_set = IntervalSet::new(intervals);
    interval_set.is_canonical(); // This should evaluate to false
}

#[test]
fn test_is_canonical_with_adjacent_intervals() {
    let intervals = vec![
        TestInterval { lower: TestBound(1), upper: TestBound(2) },
        TestInterval { lower: TestBound(2), upper: TestBound(3) }, // Adjacent and contiguous
        TestInterval { lower: TestBound(3), upper: TestBound(4) },
    ];

    let interval_set = IntervalSet::new(intervals);
    interval_set.is_canonical(); // This should evaluate to false
}

