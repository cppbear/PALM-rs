// Answer 0

#[test]
fn test_difference_self_ranges_empty_other_ranges_non_empty() {
    let mut self_set = IntervalSet::<MyInterval>::new(vec![]);
    let other_set = IntervalSet::<MyInterval>::new(vec![MyInterval::create(1, 5)]);
    self_set.difference(&other_set);
}

#[test]
fn test_difference_self_ranges_empty_other_ranges_empty() {
    let mut self_set = IntervalSet::<MyInterval>::new(vec![]);
    let other_set = IntervalSet::<MyInterval>::new(vec![]);
    self_set.difference(&other_set);
}

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct MyInterval {
    lower_bound: i32,
    upper_bound: i32,
}

impl Interval for MyInterval {
    type Bound = i32;

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

    fn is_contiguous(&self, _other: &Self) -> bool {
        true
    }

    fn is_intersection_empty(&self, _other: &Self) -> bool {
        false
    }

    fn is_subset(&self, _other: &Self) -> bool {
        false
    }
}

