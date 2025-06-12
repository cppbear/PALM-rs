// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct Bound(u32);

impl Bound {
    fn min_value() -> Self {
        Bound(0)
    }

    fn max_value() -> Self {
        Bound(u32::MAX)
    }

    fn increment(self) -> Self {
        Bound(self.0 + 1)
    }

    fn decrement(self) -> Self {
        Bound(self.0 - 1)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct TestInterval {
    lower: Bound,
    upper: Bound,
}

impl Interval for TestInterval {
    type Bound = Bound;

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

    fn is_contiguous(&self, _: &Self) -> bool {
        false
    }

    fn is_intersection_empty(&self, _: &Self) -> bool {
        false
    }

    fn is_subset(&self, _: &Self) -> bool {
        false
    }
}

#[test]
fn test_negate_empty_ranges() {
    let mut interval_set = IntervalSet::<TestInterval>::new(vec![]);
    interval_set.negate();
}

#[test]
fn test_negate_single_range() {
    let mut interval_set = IntervalSet::<TestInterval>::new(vec![
        TestInterval { lower: Bound(10), upper: Bound(20) }
    ]);
    interval_set.negate();
}

#[test]
fn test_negate_multiple_ranges() {
    let mut interval_set = IntervalSet::<TestInterval>::new(vec![
        TestInterval { lower: Bound(5), upper: Bound(10) },
        TestInterval { lower: Bound(15), upper: Bound(25) }
    ]);
    interval_set.negate();
}

