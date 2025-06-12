// Answer 0

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct TestBound(u8);

impl TestBound {
    fn new(value: u8) -> Self {
        TestBound(value)
    }
    
    fn decrement(self) -> Self {
        TestBound(self.0.wrapping_sub(1))
    }

    fn increment(self) -> Self {
        TestBound(self.0.wrapping_add(1))
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
        self.upper.0 + 1 == other.lower.0 || self.lower.0 == other.upper.0 + 1
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper.0 < other.lower.0 || self.lower.0 > other.upper.0
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
    }
}

#[test]
fn test_difference_no_intersection() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(3) },
        TestInterval { lower: TestBound::new(5), upper: TestBound::new(7) }
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound::new(8), upper: TestBound::new(10) }
    ]);
    
    set_a.difference(&set_b);
    
    assert_eq!(set_a.intervals(), &[
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(3) },
        TestInterval { lower: TestBound::new(5), upper: TestBound::new(7) }
    ]);
}

#[test]
fn test_difference_full_overlap() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(5) }
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound::new(2), upper: TestBound::new(3) }
    ]);
    
    set_a.difference(&set_b);
    
    assert_eq!(set_a.intervals(), &[
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(2) },
        TestInterval { lower: TestBound::new(3), upper: TestBound::new(5) }
    ]);
}

#[test]
fn test_difference_adjacent_no_touch() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(3) },
        TestInterval { lower: TestBound::new(4), upper: TestBound::new(6) }
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound::new(7), upper: TestBound::new(8) }
    ]);

    set_a.difference(&set_b);
    
    assert_eq!(set_a.intervals(), &[
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(3) },
        TestInterval { lower: TestBound::new(4), upper: TestBound::new(6) }
    ]);
}

#[test]
fn test_difference_with_non_contiguous() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(10) }
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound::new(3), upper: TestBound::new(5) },
        TestInterval { lower: TestBound::new(8), upper: TestBound::new(9) }
    ]);

    set_a.difference(&set_b);
    
    assert_eq!(set_a.intervals(), &[
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(3) },
        TestInterval { lower: TestBound::new(5), upper: TestBound::new(8) },
        TestInterval { lower: TestBound::new(10), upper: TestBound::new(10) }
    ]);
}

#[test]
fn test_difference_with_empty() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(5) }
    ]);
    let set_b = IntervalSet::new(vec![]);

    set_a.difference(&set_b);
    
    assert_eq!(set_a.intervals(), &[
        TestInterval { lower: TestBound::new(1), upper: TestBound::new(5) }
    ]);
}

