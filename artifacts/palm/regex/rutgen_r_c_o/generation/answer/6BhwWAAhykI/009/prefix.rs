// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(u8);

impl TestBound {
    fn increment(&self) -> Self {
        TestBound(self.0 + 1)
    }
    fn decrement(&self) -> Self {
        TestBound(self.0 - 1)
    }
}

impl std::fmt::Display for TestBound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
fn test_intersect_non_empty() {
    let mut set_a = IntervalSet::new(vec![TestInterval { lower: TestBound(0), upper: TestBound(10) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: TestBound(5), upper: TestBound(15) }]);
    set_a.intersect(&set_b);
}

#[test]
fn test_intersect_equal_upper() {
    let mut set_a = IntervalSet::new(vec![TestInterval { lower: TestBound(0), upper: TestBound(10) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: TestBound(10), upper: TestBound(15) }]);
    set_a.intersect(&set_b);
}

#[test]
fn test_intersect_gap() {
    let mut set_a = IntervalSet::new(vec![TestInterval { lower: TestBound(0), upper: TestBound(5) }]);
    let set_b = IntervalSet::new(vec![TestInterval { lower: TestBound(6), upper: TestBound(15) }]);
    set_a.intersect(&set_b);
}

#[test]
fn test_intersect_multiple_ranges() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(0), upper: TestBound(10) },
        TestInterval { lower: TestBound(20), upper: TestBound(30) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(5), upper: TestBound(15) },
        TestInterval { lower: TestBound(25), upper: TestBound(35) },
    ]);
    set_a.intersect(&set_b);
}

#[test]
fn test_intersect_with_empty_b() {
    let mut set_a = IntervalSet::new(vec![TestInterval { lower: TestBound(0), upper: TestBound(10) }]);
    let set_b = IntervalSet::new(vec![]);
    set_a.intersect(&set_b);
}

