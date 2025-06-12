// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct TestBound(i32);

impl TestBound {
    fn increment(&self) -> Self {
        TestBound(self.0 + 1)
    }

    fn decrement(&self) -> Self {
        TestBound(self.0 - 1)
    }
}

impl Bound for TestBound {}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
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
        self.lower() <= other.upper() && other.lower() <= self.upper()
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        self.upper() < other.lower() || self.lower() > other.upper()
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower() >= other.lower() && self.upper() <= other.upper()
    }
}

#[test]
fn test_symmetric_difference_case_1() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(-100), upper: TestBound(0) },
        TestInterval { lower: TestBound(10), upper: TestBound(20) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(-50), upper: TestBound(15) },
    ]);
    set_a.symmetric_difference(&set_b);
}

#[test]
fn test_symmetric_difference_case_2() {
    let mut set_a = IntervalSet::new(vec![]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(0), upper: TestBound(100) },
    ]);
    set_a.symmetric_difference(&set_b);
}

#[test]
fn test_symmetric_difference_case_3() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(1), upper: TestBound(3) },
        TestInterval { lower: TestBound(5), upper: TestBound(7) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(2), upper: TestBound(6) },
    ]);
    set_a.symmetric_difference(&set_b);
}

#[test]
fn test_symmetric_difference_case_4() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(100), upper: TestBound(200) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(150), upper: TestBound(250) },
    ]);
    set_a.symmetric_difference(&set_b);
}

#[test]
fn test_symmetric_difference_case_5() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(500), upper: TestBound(600) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(600), upper: TestBound(700) },
    ]);
    set_a.symmetric_difference(&set_b);
}

#[test]
fn test_symmetric_difference_case_6() {
    let mut set_a = IntervalSet::new(vec![
        TestInterval { lower: TestBound(-1000), upper: TestBound(-900) },
    ]);
    let set_b = IntervalSet::new(vec![
        TestInterval { lower: TestBound(-950), upper: TestBound(-850) },
    ]);
    set_a.symmetric_difference(&set_b);
}

