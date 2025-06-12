// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(u8);

impl TestBound {
    fn min_value() -> Self {
        TestBound(0)
    }
    
    fn max_value() -> Self {
        TestBound(u8::MAX)
    }
    
    fn decrement(self) -> Self {
        TestBound(self.0.saturating_sub(1))
    }
    
    fn increment(self) -> Self {
        TestBound(self.0.saturating_add(1))
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
    
    fn case_fold_simple(&self, intervals: &mut Vec<Self>) {}
    
    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper >= other.lower || self.lower <= other.upper
    }
    
    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.lower > other.upper || self.upper < other.lower
    }
    
    fn is_subset(&self, other: &Self) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

#[test]
fn test_negate_empty() {
    let mut interval_set: IntervalSet<TestInterval> = IntervalSet::new(vec![]);
    interval_set.negate();
    assert_eq!(interval_set.intervals().len(), 1);
    assert_eq!(interval_set.intervals()[0], TestInterval { lower: TestBound::min_value(), upper: TestBound::max_value() });
}

#[test]
fn test_negate_single_interval() {
    let mut interval_set = IntervalSet::new(vec![TestInterval { lower: TestBound(10), upper: TestBound(20) }]);
    interval_set.negate();
    assert_eq!(interval_set.intervals().len(), 2);
    assert_eq!(interval_set.intervals()[0], TestInterval { lower: TestBound::min_value(), upper: TestBound(9) });
    assert_eq!(interval_set.intervals()[1], TestInterval { lower: TestBound(21), upper: TestBound::max_value() });
}

#[test]
fn test_negate_multiple_intervals() {
    let mut interval_set = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(5), upper: TestBound(7) }
    ]);
    interval_set.negate();
    assert_eq!(interval_set.intervals().len(), 3);
    assert_eq!(interval_set.intervals()[0], TestInterval { lower: TestBound::min_value(), upper: TestBound(0) });
    assert_eq!(interval_set.intervals()[1], TestInterval { lower: TestBound(4), upper: TestBound(4) });
    assert_eq!(interval_set.intervals()[2], TestInterval { lower: TestBound(8), upper: TestBound::max_value() });
}

#[test]
#[should_panic]
fn test_negate_panic_on_lower_boundary() {
    let mut interval_set = IntervalSet::new(vec![TestInterval { lower: TestBound::min_value(), upper: TestBound(5) }]);
    interval_set.negate();
}

#[test]
#[should_panic]
fn test_negate_panic_on_upper_boundary() {
    let mut interval_set = IntervalSet::new(vec![TestInterval { lower: TestBound(0), upper: TestBound::max_value() }]);
    interval_set.negate();
}

