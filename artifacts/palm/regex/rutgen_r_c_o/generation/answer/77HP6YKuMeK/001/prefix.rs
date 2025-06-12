// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct TestInterval {
    lower: u8,
    upper: u8,
}

impl TestInterval {
    fn new(lower: u8, upper: u8) -> Self {
        Self { lower, upper }
    }
}

impl Interval for TestInterval {
    type Bound = u8;

    fn lower(&self) -> Self::Bound { self.lower }
    fn upper(&self) -> Self::Bound { self.upper }
    fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
    fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
    fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}
    fn is_contiguous(&self, other: &Self) -> bool { self.upper >= other.lower && self.lower <= other.upper }
    fn is_intersection_empty(&self, other: &Self) -> bool { self.upper < other.lower || self.lower > other.upper }
    fn is_subset(&self, other: &Self) -> bool { self.lower >= other.lower && self.upper <= other.upper }
}

#[test]
fn test_intervals_empty() {
    let interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![]);
    let _ = interval_set.intervals();
}

#[test]
fn test_intervals_single_element() {
    let interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![TestInterval::new(1, 2)]);
    let _ = interval_set.intervals();
}

#[test]
fn test_intervals_multiple_elements() {
    let interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![
        TestInterval::new(0, 2),
        TestInterval::new(3, 5),
        TestInterval::new(6, 10),
    ]);
    let _ = interval_set.intervals();
}

#[test]
fn test_intervals_full_cover() {
    let interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![
        TestInterval::new(0, 255),
    ]);
    let _ = interval_set.intervals();
}

#[test]
fn test_intervals_edge_case() {
    let interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![
        TestInterval::new(10, 20),
        TestInterval::new(20, 30),
    ]);
    let _ = interval_set.intervals();
}

#[test]
fn test_intervals_max_elements() {
    let intervals = (0..1000).map(|i| TestInterval::new(i % 256, (i + 1) % 256)).collect::<Vec<_>>();
    let interval_set: IntervalSet<TestInterval> = IntervalSet::new(intervals);
    let _ = interval_set.intervals();
}

