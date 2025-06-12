// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(u8);

impl TestBound {
    fn increment(self) -> Self {
        TestBound(self.0 + 1)
    }
    fn decrement(self) -> Self {
        TestBound(self.0.saturating_sub(1))
    }
}

impl Bound for TestBound {}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
        self.upper >= other.lower && self.lower <= other.upper
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.lower > other.upper || self.upper < other.lower
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_intersect_non_empty_sets() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(5),
        },
        TestInterval {
            lower: TestBound(10),
            upper: TestBound(15),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(4),
            upper: TestBound(12),
        },
        TestInterval {
            lower: TestBound(13),
            upper: TestBound(18),
        },
    ]);

    set_a.intersect(&set_b);
    assert_eq!(set_a.intervals(), &[
        TestInterval {
            lower: TestBound(4),
            upper: TestBound(5),
        },
        TestInterval {
            lower: TestBound(10),
            upper: TestBound(12),
        },
    ]);
}

#[test]
fn test_intersect_with_no_overlap() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(3),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(4),
            upper: TestBound(5),
        },
    ]);

    set_a.intersect(&set_b);
    assert_eq!(set_a.intervals(), &[]);
}

#[test]
fn test_intersect_with_empty_set() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(5),
        },
    ]);

    let set_b = IntervalSet::new(vec![]); // Empty set

    set_a.intersect(&set_b);
    assert_eq!(set_a.intervals(), &[]);
}

#[test]
fn test_intersect_with_identical_intervals() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(5),
        },
    ]);

    let set_b = IntervalSet::new(vec![
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(5),
        },
    ]);

    set_a.intersect(&set_b);
    assert_eq!(set_a.intervals(), &[
        TestInterval {
            lower: TestBound(1),
            upper: TestBound(5),
        },
    ]);
}

