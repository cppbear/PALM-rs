// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct Bound(u8);

impl Bound {
    fn decrement(self) -> Self {
        Bound(self.0.saturating_sub(1))
    }

    fn increment(self) -> Self {
        Bound(self.0.saturating_add(1))
    }
}

impl Debug for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

impl PartialEq for Bound {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Bound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

trait IntervalTest: Interval {
}

impl IntervalTest for Bound {
    fn lower(&self) -> Self {
        *self
    }

    fn upper(&self) -> Self {
        *self
    }

    fn set_lower(&mut self, bound: Self) {
        self.0 = bound.0;
    }

    fn set_upper(&mut self, bound: Self) {
        self.0 = bound.0;
    }

    fn case_fold_simple(&self, _: &mut Vec<Self>) {
        // No implementation needed for this test
    }

    fn is_contiguous(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.0 <= other.0
    }
}

#[test]
fn test_difference_non_empty_ranges() {
    let mut set_a = IntervalSet::new(vec![Bound(1), Bound(3), Bound(5)]);
    let set_b = IntervalSet::new(vec![Bound(2), Bound(4)]);

    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), &[Bound(1), Bound(5)]);
}

#[test]
fn test_difference_exclude_all() {
    let mut set_a = IntervalSet::new(vec![Bound(1), Bound(5)]);
    let set_b = IntervalSet::new(vec![Bound(1), Bound(5)]);

    set_a.difference(&set_b);

    assert!(set_a.intervals().is_empty());
}

#[test]
fn test_difference_with_empty_subtraction() {
    let mut set_a = IntervalSet::new(vec![Bound(1), Bound(3), Bound(5)]);
    let set_b = IntervalSet::new(vec![Bound(6)]);

    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), &[Bound(1), Bound(3), Bound(5)]);
}

#[test]
fn test_difference_multiple_intersections() {
    let mut set_a = IntervalSet::new(vec![Bound(1), Bound(6)]);
    let set_b = IntervalSet::new(vec![Bound(2), Bound(3), Bound(4)]);

    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), &[Bound(1), Bound(6)]);
}

#[test]
#[should_panic]
fn test_difference_empty_self() {
    let mut set_a: IntervalSet<Bound> = IntervalSet::new(vec![]);
    let set_b = IntervalSet::new(vec![Bound(1), Bound(2)]);

    set_a.difference(&set_b);
}

#[test]
#[should_panic]
fn test_difference_empty_other() {
    let mut set_a = IntervalSet::new(vec![Bound(1), Bound(3)]);
    let set_b: IntervalSet<Bound> = IntervalSet::new(vec![]);

    set_a.difference(&set_b);
}

