// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct SimpleInterval {
    lower: i32,
    upper: i32,
}

impl SimpleInterval {
    fn new(lower: i32, upper: i32) -> Self {
        SimpleInterval { lower, upper }
    }
}

impl Interval for SimpleInterval {
    type Bound = i32;

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
        self.upper + 1 >= other.lower && other.upper + 1 >= self.lower
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || other.upper < self.lower
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_is_canonical_true() {
    let intervals = vec![
        SimpleInterval::new(1, 2),
        SimpleInterval::new(4, 5),
        SimpleInterval::new(7, 8),
    ];
    let interval_set = IntervalSet::new(intervals);
    let result = interval_set.is_canonical();
}

#[test]
fn test_is_canonical_false_pair_greater() {
    let intervals = vec![
        SimpleInterval::new(1, 3),
        SimpleInterval::new(2, 5),
    ];
    let interval_set = IntervalSet::new(intervals);
    let result = interval_set.is_canonical();
}

#[test]
fn test_is_canonical_false_contiguous() {
    let intervals = vec![
        SimpleInterval::new(1, 3),
        SimpleInterval::new(3, 5),
    ];
    let interval_set = IntervalSet::new(intervals);
    let result = interval_set.is_canonical();
}

#[test]
fn test_is_canonical_empty() {
    let intervals: Vec<SimpleInterval> = vec![];
    let interval_set = IntervalSet::new(intervals);
    let result = interval_set.is_canonical();
}

#[test]
fn test_is_canonical_single_interval() {
    let intervals = vec![
        SimpleInterval::new(2, 4),
    ];
    let interval_set = IntervalSet::new(intervals);
    let result = interval_set.is_canonical();
}

