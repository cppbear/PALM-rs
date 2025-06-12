// Answer 0

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestInterval {
    lower: usize,
    upper: usize,
}

impl Default for TestInterval {
    fn default() -> Self {
        TestInterval { lower: 0, upper: 0 }
    }
}

impl Interval for TestInterval {
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
fn test_is_canonical_empty_ranges() {
    let interval_set = IntervalSet::<TestInterval>::new(vec![]);
    interval_set.is_canonical();
}

#[test]
fn test_is_canonical_single_range() {
    let interval = TestInterval { lower: 1, upper: 5 };
    let interval_set = IntervalSet::new(vec![interval]);
    interval_set.is_canonical();
}

