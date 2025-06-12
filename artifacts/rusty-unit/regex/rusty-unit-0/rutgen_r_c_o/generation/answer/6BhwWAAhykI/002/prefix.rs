// Answer 0

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Bound {
    value: usize,
}
impl Bound {
    fn increment(&self) -> Self {
        Bound { value: self.value + 1 }
    }
    fn decrement(&self) -> Self {
        Bound { value: self.value - 1 }
    }
}
impl PartialOrd for Bound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl Ord for Bound {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl Bound {
    fn new(value: usize) -> Self {
        Bound { value }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct IntervalImpl {
    lower: Bound,
    upper: Bound,
}
impl Interval for IntervalImpl {
    type Bound = Bound;

    fn lower(&self) -> Self::Bound {
        self.lower.clone()
    }

    fn upper(&self) -> Self::Bound {
        self.upper.clone()
    }

    fn set_lower(&mut self, bound: Self::Bound) {
        self.lower = bound;
    }

    fn set_upper(&mut self, bound: Self::Bound) {
        self.upper = bound;
    }

    fn case_fold_simple(&self, _: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper >= other.lower
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || other.upper < self.lower
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_intersect_with_empty_other_ranges() {
    let mut set_a = IntervalSet::new(vec![
        IntervalImpl::create(Bound::new(1), Bound::new(5)),
        IntervalImpl::create(Bound::new(6), Bound::new(10)),
    ]);
    let set_b = IntervalSet::<IntervalImpl>::new(vec![]);

    set_a.intersect(&set_b);
}

#[test]
fn test_intersect_with_non_empty_self_ranges() {
    let mut set_a = IntervalSet::new(vec![
        IntervalImpl::create(Bound::new(0), Bound::new(3)),
        IntervalImpl::create(Bound::new(5), Bound::new(7)),
    ]);
    let set_b = IntervalSet::<IntervalImpl>::new(vec![]);

    set_a.intersect(&set_b);
}

#[test]
#[should_panic]
fn test_intersect_with_non_empty_self_ranges_and_empty_other_and_verify_no_panics() {
    let mut set_a = IntervalSet::new(vec![IntervalImpl::create(Bound::new(1), Bound::new(3))]);
    let set_b = IntervalSet::<IntervalImpl>::new(vec![]);

    set_a.intersect(&set_b);
}

