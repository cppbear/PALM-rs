// Answer 0

#[derive(Clone, Debug, Eq, PartialEq)]
struct Bound {
    value: i32,
}

impl Bound {
    fn min_value() -> Self {
        Bound { value: i32::MIN }
    }

    fn max_value() -> Self {
        Bound { value: i32::MAX }
    }

    fn increment(&self) -> Self {
        Bound { value: self.value + 1 }
    }

    fn decrement(&self) -> Self {
        Bound { value: self.value - 1 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestInterval {
    lower_bound: Bound,
    upper_bound: Bound,
}

impl Interval for TestInterval {
    type Bound = Bound;

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
        self.upper() >= other.lower() && other.upper() >= self.lower()
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper() < other.lower() || other.upper() < self.lower()
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower() >= other.lower() && self.upper() <= other.upper()
    }
}

#[test]
fn test_negate_empty_set() {
    let mut interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![]);
    interval_set.negate();
    assert_eq!(interval_set.intervals(), &[TestInterval {
        lower_bound: Bound::min_value(),
        upper_bound: Bound::max_value(),
    }]);
}

#[test]
fn test_negate_single_element() {
    let mut interval_set = IntervalSet::new(vec![TestInterval {
        lower_bound: Bound { value: 1 },
        upper_bound: Bound { value: 3 },
    }]);
    interval_set.negate();
    
    assert_eq!(interval_set.intervals(), &[
        TestInterval {
            lower_bound: Bound::min_value(),
            upper_bound: Bound { value: 0 },
        },
        TestInterval {
            lower_bound: Bound { value: 4 },
            upper_bound: Bound::max_value(),
        }
    ]);
}

#[test]
fn test_negate_multiple_elements() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval {
            lower_bound: Bound { value: 1 },
            upper_bound: Bound { value: 3 },
        },
        TestInterval {
            lower_bound: Bound { value: 5 },
            upper_bound: Bound { value: 7 },
        },
    ]);
    
    interval_set.negate();
    
    assert_eq!(interval_set.intervals(), &[
        TestInterval {
            lower_bound: Bound::min_value(),
            upper_bound: Bound { value: 0 },
        },
        TestInterval {
            lower_bound: Bound { value: 4 },
            upper_bound: Bound { value: 4 },
        },
        TestInterval {
            lower_bound: Bound { value: 8 },
            upper_bound: Bound::max_value(),
        }
    ]);
}

#[test]
#[should_panic]
fn test_negate_empty_intervals_with_panic_condition() {
    let mut interval_set = IntervalSet::new(vec![TestInterval {
        lower_bound: Bound { value: i32::MIN },
        upper_bound: Bound { value: i32::MAX },
    }]);
    interval_set.negate();
    assert!(interval_set.intervals().is_empty());
}

#[test]
fn test_negate_same_boundaries() {
    let mut interval_set = IntervalSet::new(vec![TestInterval {
        lower_bound: Bound { value: 0 },
        upper_bound: Bound { value: 0 },
    }]);
    interval_set.negate();

    assert_eq!(interval_set.intervals(), &[
        TestInterval {
            lower_bound: Bound::min_value(),
            upper_bound: Bound { value: -1 },
        },
        TestInterval {
            lower_bound: Bound { value: 1 },
            upper_bound: Bound::max_value(),
        }
    ]);
}

