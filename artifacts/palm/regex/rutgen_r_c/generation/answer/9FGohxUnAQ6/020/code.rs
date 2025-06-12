// Answer 0

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestBound(u32);

impl TestBound {
    fn decrement(&self) -> Self {
        TestBound(self.0.saturating_sub(1))
    }
    
    fn increment(&self) -> Self {
        TestBound(self.0 + 1)
    }
}

impl PartialOrd for TestBound {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for TestBound {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

trait Bound: Copy + PartialOrd + Ord + Debug {
    fn decrement(self) -> Self;
    fn increment(self) -> Self;
}

impl Bound for TestBound {
    fn decrement(self) -> Self {
        self.decrement()
    }

    fn increment(self) -> Self {
        self.increment()
    }
}

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
        self.upper == other.lower || self.lower == other.upper
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_difference_no_overlap() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
        TestInterval { lower: TestBound(6), upper: TestBound(10) },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(11), upper: TestBound(15) },
    ]);
    
    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
        TestInterval { lower: TestBound(6), upper: TestBound(10) },
    ].as_slice());
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

    assert_eq!(set_a.intervals(), vec![
        TestInterval { lower: TestBound(1), upper: TestBound(2) },
        TestInterval { lower: TestBound(5), upper: TestBound(5) },
    ].as_slice());
}

#[test]
fn test_difference_full_overlap() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    
    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), vec![].as_slice());
}

#[test]
fn test_difference_edge_overlap() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ]);
    
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(5), upper: TestBound(6) },
    ]);
    
    set_a.difference(&set_b);

    assert_eq!(set_a.intervals(), vec![
        TestInterval { lower: TestBound(1), upper: TestBound(5) },
    ].as_slice());
}

