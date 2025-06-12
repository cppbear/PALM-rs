// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct BoundI {
    value: i32,
}

impl Bound for BoundI {
    fn min_value() -> Self {
        BoundI { value: i32::MIN }
    }

    fn max_value() -> Self {
        BoundI { value: i32::MAX }
    }

    fn decrement(&self) -> Self {
        BoundI { value: self.value - 1 }
    }

    fn increment(&self) -> Self {
        BoundI { value: self.value + 1 }
    }

    fn is_less_than(&self, other: &Self) -> bool {
        self.value < other.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IntervalI {
    lower_bound: BoundI,
    upper_bound: BoundI,
}

impl Interval for IntervalI {
    type Bound = BoundI;

    fn lower(&self) -> Self::Bound {
        self.lower_bound
    }

    fn upper(&self) -> Self::Bound {
        self.upper_bound
    }

    fn set_lower(&mut self, bound: Self::Bound) {
        self.lower_bound = bound;
    }

    fn set_upper(&mut self, bound: Self::Bound) {
        self.upper_bound = bound;
    }

    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper() >= other.lower() && self.lower() <= other.upper()
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper() < other.lower() || self.lower() > other.upper()
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower() >= other.lower() && self.upper() <= other.upper()
    }
}

#[test]
fn test_negate_non_empty_with_boundary_conditions() {
    let mut interval_set = IntervalSet::new(vec![
        IntervalI { lower_bound: BoundI { value: 1 }, upper_bound: BoundI { value: 5 }},
        IntervalI { lower_bound: BoundI { value: 10 }, upper_bound: BoundI { value: 15 }},
    ]);

    // Verify the set before negation
    assert_eq!(interval_set.intervals(), &[ 
        IntervalI { lower_bound: BoundI { value: 1 }, upper_bound: BoundI { value: 5 }},
        IntervalI { lower_bound: BoundI { value: 10 }, upper_bound: BoundI { value: 15 }},
    ]);

    interval_set.negate();

    // After negation, we should have intervals that do not overlap with original
    assert_eq!(interval_set.intervals(), &[
        IntervalI { lower_bound: BoundI::min_value(), upper_bound: BoundI { value: 0 }},
        IntervalI { lower_bound: BoundI { value: 6 }, upper_bound: BoundI { value: 9 }},
        IntervalI { lower_bound: BoundI { value: 16 }, upper_bound: BoundI::max_value() },
    ]);
}

#[test]
fn test_negate_empty_intervals() {
    let mut interval_set = IntervalSet::new(vec![]);

    // Verify the set before negation
    assert_eq!(interval_set.intervals(), &[]);

    interval_set.negate();

    // After negation, it should cover all possible bounds
    assert_eq!(interval_set.intervals(), &[
        IntervalI { lower_bound: BoundI::min_value(), upper_bound: BoundI::max_value() },
    ]);
}

#[test]
fn test_negate_only_one_interval() {
    let mut interval_set = IntervalSet::new(vec![
        IntervalI { lower_bound: BoundI { value: 5 }, upper_bound: BoundI { value: 10 }},
    ]);

    interval_set.negate();

    // Should negate the single interval
    assert_eq!(interval_set.intervals(), &[
        IntervalI { lower_bound: BoundI::min_value(), upper_bound: BoundI { value: 4 }},
        IntervalI { lower_bound: BoundI { value: 11 }, upper_bound: BoundI::max_value() },
    ]);
}

