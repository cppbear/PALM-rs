// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct TestBound(u32);

impl TestBound {
    fn increment(self) -> Self {
        TestBound(self.0 + 1)
    }

    fn decrement(self) -> Self {
        TestBound(self.0 - 1)
    }
}

impl std::fmt::Debug for TestBound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TestBound({})", self.0)
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

    fn case_fold_simple(&self, intervals: &mut Vec<Self>) {
        intervals.push(self.clone());
    }

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
fn test_intersect_empty_self() {
    let mut set_a = IntervalSet::new(vec![]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    set_a.intersect(&set_b);
    assert!(set_a.ranges.is_empty());
}

#[test]
fn test_intersect_empty_other() {
    let mut set_a = IntervalSet::new(vec![TestInterval { lower: TestBound(1), upper: TestBound(5) }]);
    let set_b = IntervalSet::new(vec![]);
    set_a.intersect(&set_b);
    assert!(set_a.ranges.is_empty());
}

#[test]
fn test_intersect_non_empty() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(5), upper: TestBound(7) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(2), upper: TestBound(6) },
    ]);
    set_a.intersect(&set_b);
    assert_eq!(set_a.ranges.len(), 2);
    assert_eq!(set_a.ranges[0], TestInterval { lower: TestBound(2), upper: TestBound(3) });
    assert_eq!(set_a.ranges[1], TestInterval { lower: TestBound(5), upper: TestBound(6) });
}

#[test]
fn test_intersect_no_overlap() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(2) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(3), upper: TestBound(4) },
    ]);
    set_a.intersect(&set_b);
    assert!(set_a.ranges.is_empty());
}

