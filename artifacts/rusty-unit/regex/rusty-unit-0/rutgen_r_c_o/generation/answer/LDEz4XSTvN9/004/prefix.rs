// Answer 0

#[derive(Clone, Debug, Eq, PartialEq)]
struct IntBound {
    value: i32,
}

impl IntBound {
    fn min_value() -> Self {
        IntBound { value: i32::MIN }
    }
    
    fn max_value() -> Self {
        IntBound { value: i32::MAX }
    }
    
    fn decrement(&self) -> Self {
        IntBound { value: self.value - 1 }
    }
    
    fn increment(&self) -> Self {
        IntBound { value: self.value + 1 }
    }
}

impl PartialOrd for IntBound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for IntBound {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl Default for IntBound {
    fn default() -> Self {
        IntBound { value: 0 }
    }
}

impl Eq for IntBound {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestInterval {
    lower_bound: IntBound,
    upper_bound: IntBound,
}

impl Interval for TestInterval {
    type Bound = IntBound;

    fn lower(&self) -> Self::Bound {
        self.lower_bound.clone()
    }
    
    fn upper(&self) -> Self::Bound {
        self.upper_bound.clone()
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
fn test_negate_with_multiple_elements() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower_bound: IntBound { value: 1 }, upper_bound: IntBound { value: 10 } },
        TestInterval { lower_bound: IntBound { value: 20 }, upper_bound: IntBound { value: 30 } },
    ]);
    interval_set.negate();
}

#[test]
fn test_negate_with_adjacent_intervals() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower_bound: IntBound { value: 5 }, upper_bound: IntBound { value: 15 } },
        TestInterval { lower_bound: IntBound { value: 20 }, upper_bound: IntBound { value: 25 } },
    ]);
    interval_set.negate();
}

#[test]
fn test_negate_with_intervals_at_limits() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower_bound: IntBound { value: IntBound::min_value().value + 1 }, upper_bound: IntBound { value: IntBound::max_value().value - 1 } },
        TestInterval { lower_bound: IntBound { value: IntBound::max_value().value - 5 }, upper_bound: IntBound { value: IntBound::max_value().value - 2 } },
    ]);
    interval_set.negate();
}

#[test]
fn test_negate_with_single_element() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower_bound: IntBound { value: 3 }, upper_bound: IntBound { value: 5 } },
    ]);
    interval_set.negate();
}

#[test]
fn test_negate_with_no_range() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower_bound: IntBound::min_value(), upper_bound: IntBound { value: IntBound::max_value().value - 1 } },
    ]);
    interval_set.negate();
}

