// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(u32);

impl TestBound {
    fn increment(self) -> Self {
        TestBound(self.0 + 1)
    }
    fn decrement(self) -> Self {
        TestBound(self.0 - 1)
    }
}

impl Bound for TestBound {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestInterval {
    lower: TestBound,
    upper: TestBound,
}

impl Default for TestInterval {
    fn default() -> Self {
        TestInterval {
            lower: TestBound(0),
            upper: TestBound(0),
        }
    }
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

    fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper >= other.lower
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.lower > other.upper || self.upper < other.lower
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_push_new_interval() {
    let mut set = IntervalSet::new(vec![]);
    let interval = TestInterval {
        lower: TestBound(1),
        upper: TestBound(3),
    };
    set.push(interval);
    assert_eq!(set.intervals().len(), 1);
    assert_eq!(set.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(3) });
}

#[test]
fn test_push_overlapping_intervals() {
    let mut set = IntervalSet::new(vec![]);
    let interval1 = TestInterval {
        lower: TestBound(1),
        upper: TestBound(5),
    };
    let interval2 = TestInterval {
        lower: TestBound(3),
        upper: TestBound(7),
    };
    set.push(interval1);
    set.push(interval2);
    assert_eq!(set.intervals().len(), 1);
    assert_eq!(set.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(7) });
}

#[test]
fn test_push_non_contiguous_intervals() {
    let mut set = IntervalSet::new(vec![]);
    let interval1 = TestInterval {
        lower: TestBound(1),
        upper: TestBound(2),
    };
    let interval2 = TestInterval {
        lower: TestBound(4),
        upper: TestBound(5),
    };
    set.push(interval1);
    set.push(interval2);
    assert_eq!(set.intervals().len(), 2);
    assert_eq!(set.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(2) });
    assert_eq!(set.intervals()[1], TestInterval { lower: TestBound(4), upper: TestBound(5) });
}

#[test]
fn test_push_edge_case_same_interval() {
    let mut set = IntervalSet::new(vec![]);
    let interval = TestInterval {
        lower: TestBound(2),
        upper: TestBound(2),
    };
    set.push(interval);
    assert_eq!(set.intervals().len(), 1);
    assert_eq!(set.intervals()[0], TestInterval { lower: TestBound(2), upper: TestBound(2) });
}

#[test]
fn test_push_edge_case_reverse_order() {
    let mut set = IntervalSet::new(vec![]);
    let interval = TestInterval {
        lower: TestBound(3),
        upper: TestBound(1),
    };
    set.push(interval);
    assert_eq!(set.intervals().len(), 1);
    assert_eq!(set.intervals()[0], TestInterval { lower: TestBound(1), upper: TestBound(3) });
}

