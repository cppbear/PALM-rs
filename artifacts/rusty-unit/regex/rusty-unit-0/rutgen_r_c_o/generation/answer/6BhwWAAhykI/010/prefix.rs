// Answer 0

#[test]
fn test_intersect_non_empty_sets() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound {
        value: i32,
    }

    impl Bound {
        fn increment(&self) -> Self {
            Self { value: self.value + 1 }
        }

        fn decrement(&self) -> Self {
            Self { value: self.value - 1 }
        }
    }

    impl std::ops::Add for Bound {
        type Output = Self;
        fn add(self, _other: Self) -> Self::Output {
            self.increment()
        }
    }

    impl std::ops::Sub for Bound {
        type Output = Self;
        fn sub(self, _other: Self) -> Self::Output {
            self.decrement()
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct IntervalImpl {
        lower: Bound,
        upper: Bound,
    }

    impl Interval for IntervalImpl {
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

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper.value >= other.lower.value
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.value < other.lower.value || self.lower.value > other.upper.value
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower.value >= other.lower.value && self.upper.value <= other.upper.value
        }
    }

    let mut set_a = IntervalSet::new(vec![
        IntervalImpl { lower: Bound { value: 1 }, upper: Bound { value: 3 } },
    ]);
    let set_b = IntervalSet::new(vec![
        IntervalImpl { lower: Bound { value: 2 }, upper: Bound { value: 4 } },
    ]);

    set_a.intersect(&set_b);
}

#[test]
fn test_intersect_with_identical_ranges() {
    let mut set_a = IntervalSet::new(vec![
        IntervalImpl { lower: Bound { value: 2 }, upper: Bound { value: 3 } },
    ]);
    let set_b = IntervalSet::new(vec![
        IntervalImpl { lower: Bound { value: 2 }, upper: Bound { value: 3 } },
    ]);

    set_a.intersect(&set_b);
}

#[test]
fn test_intersect_with_non_contiguous_ranges() {
    let mut set_a = IntervalSet::new(vec![
        IntervalImpl { lower: Bound { value: 1 }, upper: Bound { value: 2 } },
    ]);
    let set_b = IntervalSet::new(vec![
        IntervalImpl { lower: Bound { value: 3 }, upper: Bound { value: 4 } },
    ]);

    set_a.intersect(&set_b);
}

#[test]
fn test_intersect_with_overlapping_ranges() {
    let mut set_a = IntervalSet::new(vec![
        IntervalImpl { lower: Bound { value: 1 }, upper: Bound { value: 5 } },
    ]);
    let set_b = IntervalSet::new(vec![
        IntervalImpl { lower: Bound { value: 4 }, upper: Bound { value: 6 } },
    ]);

    set_a.intersect(&set_b);
}

