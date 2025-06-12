// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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
        self.upper + 1 == other.lower || self.lower == other.upper + 1
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_push_non_overlapping_intervals() {
    let mut set = IntervalSet::new(vec![]);
    set.push(TestInterval::new(10, 20));
    set.push(TestInterval::new(30, 40));
}

#[test]
fn test_push_touching_intervals() {
    let mut set = IntervalSet::new(vec![]);
    set.push(TestInterval::new(1, 1));
    set.push(TestInterval::new(2, 2));
}

#[test]
fn test_push_disjoint_intervals() {
    let mut set = IntervalSet::new(vec![]);
    set.push(TestInterval::new(5, 15));
    set.push(TestInterval::new(20, 25));
    set.push(TestInterval::new(30, 35));
}

#[test]
fn test_push_contiguous_intervals() {
    let mut set = IntervalSet::new(vec![]);
    set.push(TestInterval::new(1, 2));
    set.push(TestInterval::new(3, 4));
}

#[test]
fn test_push_with_empty_intersection() {
    let mut set = IntervalSet::new(vec![]);
    set.push(TestInterval::new(1, 10));
    set.push(TestInterval::new(20, 30));
}

#[test]
fn test_push_interval_equal_bounds() {
    let mut set = IntervalSet::new(vec![]);
    set.push(TestInterval::new(255, 255));
    set.push(TestInterval::new(0, 0));
}

