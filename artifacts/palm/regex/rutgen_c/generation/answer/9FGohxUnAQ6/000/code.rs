// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(u32);

impl TestBound {
    fn decrement(self) -> Self {
        TestBound(self.0 - 1)
    }
    
    fn increment(self) -> Self {
        TestBound(self.0 + 1)
    }
}

impl Bound for TestBound {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestInterval {
    lower: TestBound,
    upper: TestBound,
}

impl Interval for TestInterval {
    type Bound = TestBound;

    fn lower(&self) -> Self::Bound { self.lower }
    fn upper(&self) -> Self::Bound { self.upper }
    fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
    fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
    fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper() >= other.lower() && self.lower() <= other.upper()
    }
    
    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.lower() > other.upper() || self.upper() < other.lower()
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower() >= other.lower() && self.upper() <= other.upper()
    }
}

#[test]
fn test_difference_no_overlap() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(6), upper: TestBound(10) },
    ]);
    set_a.difference(&set_b);
    assert_eq!(set_a.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(5) });
}

#[test]
fn test_difference_complete_overlap() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    set_a.difference(&set_b);
    assert!(set_a.intervals().is_empty());
}

#[test]
fn test_difference_partial_overlap() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(3), upper: TestBound(4) },
    ]);
    set_a.difference(&set_b);
    assert_eq!(set_a.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(2) });
    assert_eq!(set_a.intervals()[1], TestInterval { lower: TestBound(5), upper: TestBound(5) });
}

#[test]
fn test_difference_empty_a() {
    let mut set_a = IntervalSet::new(vec![]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    set_a.difference(&set_b);
    assert!(set_a.intervals().is_empty());
}

#[test]
fn test_difference_empty_b() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    let set_b = IntervalSet::new(vec![]);
    set_a.difference(&set_b);
    assert_eq!(set_a.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(5) });
}

