// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(u32);

impl TestBound {
    fn upper(&self) -> u32 { self.0 }
    fn lower(&self) -> u32 { self.0 }
    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper() < other.lower() || self.lower() > other.upper()
    }
    fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
        if self.is_intersection_empty(other) {
            return (Some(*self), None);
        }
        if self.upper() == other.upper() && self.lower() == other.lower() {
            return (None, None);
        }
        if self.lower() >= other.lower() && self.upper() <= other.upper() {
            return (None, None);
        }
        if self.lower() < other.lower() {
            return (Some(TestBound(self.lower())), Some(TestBound(self.upper())));
        }
        (Some(TestBound(self.lower())), None)
    }
}

impl Interval for TestBound {
    type Bound = TestBound;

    fn lower(&self) -> Self::Bound { *self }
    fn upper(&self) -> Self::Bound { *self }
    fn set_lower(&mut self, bound: Self::Bound) { self.0 = bound.0; }
    fn set_upper(&mut self, bound: Self::Bound) { self.0 = bound.0; }
    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
    fn is_contiguous(&self, other: &Self) -> bool { self.upper() >= other.lower() }
    fn is_intersection_empty(&self, other: &Self) -> bool { self.upper() < other.lower() || self.lower() > other.upper() }
    fn is_subset(&self, other: &Self) -> bool { self.lower() >= other.lower() && self.upper() <= other.upper() }
}

#[test]
fn test_difference_basic() {
    let mut set_a = IntervalSet::new(vec![TestBound(5), TestBound(10)]);
    let set_b = IntervalSet::new(vec![TestBound(7)]);
    set_a.difference(&set_b);
    assert_eq!(set_a.intervals(), &[TestBound(5), TestBound(10)]);
}

#[test]
fn test_difference_non_overlapping() {
    let mut set_a = IntervalSet::new(vec![TestBound(1), TestBound(5)]);
    let set_b = IntervalSet::new(vec![TestBound(6)]);
    set_a.difference(&set_b);
    assert_eq!(set_a.intervals(), &[TestBound(1), TestBound(5)]);
}

#[test]
fn test_difference_touches() {
    let mut set_a = IntervalSet::new(vec![TestBound(1), TestBound(5)]);
    let set_b = IntervalSet::new(vec![TestBound(5)]);
    set_a.difference(&set_b);
    assert_eq!(set_a.intervals(), &[TestBound(1)]);
}

#[test]
fn test_difference_overlapping() {
    let mut set_a = IntervalSet::new(vec![TestBound(1), TestBound(5)]);
    let set_b = IntervalSet::new(vec![TestBound(3), TestBound(4)]);
    set_a.difference(&set_b);
    assert_eq!(set_a.intervals(), &[TestBound(1), TestBound(2)]);
}

#[test]
fn test_difference_entirely_removed() {
    let mut set_a = IntervalSet::new(vec![TestBound(1), TestBound(5)]);
    let set_b = IntervalSet::new(vec![TestBound(1), TestBound(5)]);
    set_a.difference(&set_b);
    assert!(set_a.intervals().is_empty());
}

#[test]
fn test_difference_multiple() {
    let mut set_a = IntervalSet::new(vec![TestBound(1), TestBound(10)]);
    let set_b = IntervalSet::new(vec![TestBound(2), TestBound(3), TestBound(9)]);
    set_a.difference(&set_b);
    assert_eq!(set_a.intervals(), &[TestBound(1), TestBound(4), TestBound(10)]);
}

