// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(i32);

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
    lower_bound: TestBound,
    upper_bound: TestBound,
}

impl Interval for TestInterval {
    type Bound = TestBound;
    
    fn lower(&self) -> Self::Bound { self.lower_bound }
    fn upper(&self) -> Self::Bound { self.upper_bound }
    fn set_lower(&mut self, bound: Self::Bound) { self.lower_bound = bound; }
    fn set_upper(&mut self, bound: Self::Bound) { self.upper_bound = bound; }
    
    fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}
    
    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper() >= other.lower() && self.lower() <= other.upper()
    }
    
    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper() < other.lower() || self.lower() > other.upper()
    }
    
    fn is_subset(&self, other: &Self) -> bool {
        self.lower() >= other.lower() && self.upper() <= other.upper()
    }
}

#[test]
fn test_canonicalize_non_canonical_single_interval() {
    let mut interval_set = IntervalSet::new(vec![TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(3) }]);
    interval_set.canonicalize();
}

#[test]
fn test_canonicalize_non_canonical_multiple_same_intervals() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(4) },
        TestInterval { lower_bound: TestBound(2), upper_bound: TestBound(5) }
    ]);
    interval_set.canonicalize();
}

#[test]
fn test_canonicalize_panic_empty_ranges() {
    let mut interval_set = IntervalSet::new(vec![]);
    #[should_panic]
    interval_set.canonicalize();
}

#[test]
fn test_canonicalize_non_canonical_one_element() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(3) }
    ]);
    interval_set.canonicalize();
}

#[test]
fn test_canonicalize_drain_empty() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower_bound: TestBound(1), upper_bound: TestBound(2) }
    ]);
    interval_set.canonicalize();
}

