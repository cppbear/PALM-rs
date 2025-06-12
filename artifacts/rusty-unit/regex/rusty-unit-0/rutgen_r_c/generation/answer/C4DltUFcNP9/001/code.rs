// Answer 0

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Bound(u32);

impl Bound {
    fn decrement(self) -> Self {
        Bound(self.0.saturating_sub(1))
    }
    
    fn increment(self) -> Self {
        Bound(self.0 + 1)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct TestInterval {
    lower_bound: Bound,
    upper_bound: Bound,
}

impl Interval for TestInterval {
    type Bound = Bound;

    fn lower(&self) -> Self::Bound {
        self.lower_bound
    }

    fn upper(&self) -> Self::Bound {
        self.upper_bound
    }

    fn set_lower(&mut self, bound: Self::Bound) {
        self.lower_bound = bound;
    }

    fn set_upper(&mut self, bound: Self::Bound) {
        self.upper_bound = bound;
    }

    fn case_fold_simple(&self, intervals: &mut Vec<Self>) {
        intervals.push(TestInterval {
            lower_bound: self.lower(),
            upper_bound: self.upper(),
        });
    }

    fn is_contiguous(&self, other: &Self) -> bool {
        self.upper() >= other.lower() && self.lower() <= other.upper()
    }

    fn is_intersection_empty(&self, other: &Self) -> bool {
        !(self.is_contiguous(other))
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.lower() >= other.lower() && self.upper() <= other.upper()
    }
}

#[derive(Clone, Debug)]
struct IntervalSetTest {
    ranges: Vec<TestInterval>,
}

impl IntervalSetTest {
    pub fn new<T: IntoIterator<Item = TestInterval>>(intervals: T) -> Self {
        Self {
            ranges: intervals.into_iter().collect(),
        }
    }

    pub fn case_fold_simple(&mut self) {
        let len = self.ranges.len();
        for i in 0..len {
            let range = self.ranges[i];
            range.case_fold_simple(&mut self.ranges);
        }
        // Simulate canonicalization as a no-op for this test.
        self.ranges.sort();
        assert!(!self.ranges.is_empty());
    }
}

#[test]
fn test_case_fold_simple_works_with_single_range() {
    let mut set = IntervalSetTest::new(vec![TestInterval {
        lower_bound: Bound(1),
        upper_bound: Bound(5),
    }]);
    set.case_fold_simple();
    assert_eq!(set.ranges.len(), 1);
    assert_eq!(set.ranges[0], TestInterval {
        lower_bound: Bound(1),
        upper_bound: Bound(5),
    });
}

#[test]
fn test_case_fold_simple_expands_single_range() {
    let mut set = IntervalSetTest::new(vec![TestInterval {
        lower_bound: Bound(10),
        upper_bound: Bound(12),
    }]);
    set.case_fold_simple();
    assert_eq!(set.ranges.len(), 1);
    assert_eq!(set.ranges[0], TestInterval {
        lower_bound: Bound(10),
        upper_bound: Bound(12),
    });
}

#[test]
fn test_case_fold_simple_with_multiple_ranges() {
    let mut set = IntervalSetTest::new(vec![
        TestInterval {
            lower_bound: Bound(3),
            upper_bound: Bound(7),
        },
        TestInterval {
            lower_bound: Bound(8),
            upper_bound: Bound(10),
        },
    ]);
    set.case_fold_simple();
    assert_eq!(set.ranges.len(), 4); // Each interval should create an additional entry
}

#[test]
#[should_panic]
fn test_case_fold_simple_panic_on_empty() {
    let mut set = IntervalSetTest::new(vec![]);
    set.case_fold_simple(); // Should panic on empty ranges
}

