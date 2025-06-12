// Answer 0

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestInterval {
    lower: u32,
    upper: u32,
}

impl Default for TestInterval {
    fn default() -> Self {
        TestInterval { lower: 0, upper: 0 }
    }
}

impl Interval for TestInterval {
    type Bound = u32;

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

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_canonicalize_multiple_overlapping_intervals() {
    let mut intervals = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 5 },
        TestInterval { lower: 3, upper: 7 },
        TestInterval { lower: 8, upper: 10 },
        TestInterval { lower: 9, upper: 12 },
    ]);
    intervals.canonicalize();
}

#[test]
fn test_canonicalize_non_overlapping_intervals() {
    let mut intervals = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 2 },
        TestInterval { lower: 3, upper: 4 },
        TestInterval { lower: 5, upper: 6 },
    ]);
    intervals.canonicalize();
}

#[test]
fn test_canonicalize_with_empty_initializations() {
    let mut intervals = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 4 },
        TestInterval { lower: 4, upper: 5 },
        TestInterval { lower: 6, upper: 10 },
    ]);
    intervals.canonicalize();
}

#[test]
fn test_canonicalize_one_long_interval() {
    let mut intervals = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 10 },
    ]);
    intervals.canonicalize();
}

#[test]
fn test_canonicalize_with_sequential_intervals() {
    let mut intervals = IntervalSet::new(vec![
        TestInterval { lower: 1, upper: 2 },
        TestInterval { lower: 2, upper: 3 },
        TestInterval { lower: 3, upper: 6 },
    ]);
    intervals.canonicalize();
}

