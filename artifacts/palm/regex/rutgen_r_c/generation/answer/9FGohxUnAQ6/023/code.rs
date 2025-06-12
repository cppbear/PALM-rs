// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Bound {
    value: i32,
}

impl Bound {
    fn increment(self) -> Self {
        Self { value: self.value + 1 }
    }
    fn decrement(self) -> Self {
        Self { value: self.value - 1 }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct MyInterval {
    lower_bound: Bound,
    upper_bound: Bound,
}

impl Interval for MyInterval {
    type Bound = Bound;

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
        self.upper() < other.lower() || other.upper() < self.lower()
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower() >= other.lower() && self.upper() <= other.upper()
    }
}

#[test]
fn test_difference_non_empty_sets() {
    let mut set1 = IntervalSet::new(vec![
        MyInterval { lower_bound: Bound { value: 1 }, upper_bound: Bound { value: 5 } },
        MyInterval { lower_bound: Bound { value: 10 }, upper_bound: Bound { value: 15 } },
    ]);
    let set2 = IntervalSet::new(vec![
        MyInterval { lower_bound: Bound { value: 3 }, upper_bound: Bound { value: 4 } },
    ]);

    set1.difference(&set2);

    let expected = vec![
        MyInterval { lower_bound: Bound { value: 1 }, upper_bound: Bound { value: 2 } },
        MyInterval { lower_bound: Bound { value: 10 }, upper_bound: Bound { value: 15 } },
    ];
    assert_eq!(set1.intervals(), &expected);
}

#[test]
fn test_difference_overlapping_ranges() {
    let mut set1 = IntervalSet::new(vec![
        MyInterval { lower_bound: Bound { value: 1 }, upper_bound: Bound { value: 5 } },
    ]);
    let set2 = IntervalSet::new(vec![
        MyInterval { lower_bound: Bound { value: 2 }, upper_bound: Bound { value: 3 } },
    ]);

    set1.difference(&set2);

    let expected = vec![
        MyInterval { lower_bound: Bound { value: 1 }, upper_bound: Bound { value: 1 } },
        MyInterval { lower_bound: Bound { value: 4 }, upper_bound: Bound { value: 5 } },
    ];
    assert_eq!(set1.intervals(), &expected);
}

#[test]
fn test_difference_with_no_common_ranges() {
    let mut set1 = IntervalSet::new(vec![
        MyInterval { lower_bound: Bound { value: 1 }, upper_bound: Bound { value: 2 } },
    ]);
    let set2 = IntervalSet::new(vec![
        MyInterval { lower_bound: Bound { value: 3 }, upper_bound: Bound { value: 4 } },
    ]);

    set1.difference(&set2);

    let expected = vec![
        MyInterval { lower_bound: Bound { value: 1 }, upper_bound: Bound { value: 2 } },
    ];
    assert_eq!(set1.intervals(), &expected);
}

#[test]
fn test_difference_empty_on_both_sets() {
    let mut set1 = IntervalSet::new(vec![
        MyInterval { lower_bound: Bound { value: 1 }, upper_bound: Bound { value: 5 } },
    ]);
    let set2 = IntervalSet::new(vec![]);

    set1.difference(&set2);

    let expected = vec![
        MyInterval { lower_bound: Bound { value: 1 }, upper_bound: Bound { value: 5 } },
    ];
    assert_eq!(set1.intervals(), &expected);
}

