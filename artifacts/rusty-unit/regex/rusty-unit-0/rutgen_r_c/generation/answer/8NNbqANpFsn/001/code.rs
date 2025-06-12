// Answer 0

#[test]
fn test_new_intervals_with_no_intervals() {
    let intervals: Vec<(u32, u32)> = Vec::new();
    let set: IntervalSet<SimpleInterval> = IntervalSet::new(intervals);
    assert!(set.intervals().is_empty());
}

#[test]
fn test_new_intervals_with_single_interval() {
    let intervals = vec![(1, 5)];
    let set: IntervalSet<SimpleInterval> = IntervalSet::new(intervals);
    let expected = vec![SimpleInterval::create(1, 5)];
    assert_eq!(set.intervals(), expected);
}

#[test]
fn test_new_intervals_with_overlapping_intervals() {
    let intervals = vec![(1, 5), (4, 8)];
    let set: IntervalSet<SimpleInterval> = IntervalSet::new(intervals);
    let expected = vec![SimpleInterval::create(1, 8)];
    assert_eq!(set.intervals(), expected);
}

#[test]
fn test_new_intervals_with_non_overlapping_intervals() {
    let intervals = vec![(1, 2), (3, 4), (5, 6)];
    let set: IntervalSet<SimpleInterval> = IntervalSet::new(intervals);
    let expected = vec![SimpleInterval::create(1, 2), SimpleInterval::create(3, 4), SimpleInterval::create(5, 6)];
    assert_eq!(set.intervals(), expected);
}

#[test]
fn test_new_intervals_with_mixed_order_and_overlapping_intervals() {
    let intervals = vec![(5, 7), (1, 3), (2, 6), (8, 10)];
    let set: IntervalSet<SimpleInterval> = IntervalSet::new(intervals);
    let expected = vec![SimpleInterval::create(1, 10)];
    assert_eq!(set.intervals(), expected);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct SimpleInterval {
    lower: u32,
    upper: u32,
}

impl SimpleInterval {
    fn create(lower: u32, upper: u32) -> Self {
        SimpleInterval { lower: lower.min(upper), upper: lower.max(upper) }
    }
}

impl Interval for SimpleInterval {
    type Bound = u32;

    fn lower(&self) -> Self::Bound { self.lower }
    fn upper(&self) -> Self::Bound { self.upper }
    fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
    fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
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

