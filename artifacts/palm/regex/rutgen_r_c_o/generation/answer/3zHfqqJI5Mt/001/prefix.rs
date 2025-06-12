// Answer 0

#[test]
fn test_iter_empty() {
    let empty_set: IntervalSet<MockInterval> = IntervalSet::new(vec![]);
    let _iter = empty_set.iter();
}

#[test]
fn test_iter_single_interval() {
    let single_interval = MockInterval::create(1, 5);
    let single_set: IntervalSet<MockInterval> = IntervalSet::new(vec![single_interval]);
    let _iter = single_set.iter();
}

#[test]
fn test_iter_multiple_intervals() {
    let interval1 = MockInterval::create(1, 3);
    let interval2 = MockInterval::create(4, 6);
    let interval3 = MockInterval::create(7, 10);
    let multiple_set: IntervalSet<MockInterval> = IntervalSet::new(vec![interval1, interval2, interval3]);
    let _iter = multiple_set.iter();
}

#[test]
fn test_iter_large_number_of_intervals() {
    let mut intervals = Vec::new();
    for i in 0..1000 {
        intervals.push(MockInterval::create(i, i + 1));
    }
    let large_set: IntervalSet<MockInterval> = IntervalSet::new(intervals);
    let _iter = large_set.iter();
}

#[test]
fn test_iter_overlapping_intervals() {
    let interval1 = MockInterval::create(1, 5);
    let interval2 = MockInterval::create(3, 7);
    let overlapping_set: IntervalSet<MockInterval> = IntervalSet::new(vec![interval1, interval2]);
    let _iter = overlapping_set.iter();
}

#[test]
fn test_iter_adjacent_intervals() {
    let interval1 = MockInterval::create(1, 5);
    let interval2 = MockInterval::create(5, 10);
    let adjacent_set: IntervalSet<MockInterval> = IntervalSet::new(vec![interval1, interval2]);
    let _iter = adjacent_set.iter();
}

#[test]
fn test_iter_same_intervals() {
    let interval = MockInterval::create(1, 5);
    let same_set: IntervalSet<MockInterval> = IntervalSet::new(vec![interval, interval, interval]);
    let _iter = same_set.iter();
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct MockInterval {
    lower: usize,
    upper: usize,
}

impl Bound for usize {
    fn increment(&self) -> usize {
        *self + 1
    }

    fn decrement(&self) -> usize {
        *self - 1
    }
}

impl Interval for MockInterval {
    type Bound = usize;

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
        self.upper >= other.lower && self.lower <= other.upper
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

