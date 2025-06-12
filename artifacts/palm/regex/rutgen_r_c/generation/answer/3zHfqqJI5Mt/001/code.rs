// Answer 0

#[test]
fn test_iter_empty() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);

    impl TestBound {
        fn increment(&self) -> TestBound {
            TestBound(self.0 + 1)
        }
        fn decrement(&self) -> TestBound {
            TestBound(self.0 - 1)
        }
    }

    impl Interval for TestBound {
        type Bound = Self;

        fn lower(&self) -> Self::Bound { *self }
        fn upper(&self) -> Self::Bound { *self }
        fn set_lower(&mut self, bound: Self::Bound) { *self = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { *self = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let set: IntervalSet<TestBound> = IntervalSet::new(vec![]);
    let mut iter = set.iter();
    
    assert_eq!(iter.0.len(), 0);
}

#[test]
fn test_iter_single() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);

    impl TestBound {
        fn increment(&self) -> TestBound {
            TestBound(self.0 + 1)
        }
        fn decrement(&self) -> TestBound {
            TestBound(self.0 - 1)
        }
    }

    impl Interval for TestBound {
        type Bound = Self;

        fn lower(&self) -> Self::Bound { *self }
        fn upper(&self) -> Self::Bound { *self }
        fn set_lower(&mut self, bound: Self::Bound) { *self = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { *self = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let interval = TestBound(5);
    let set: IntervalSet<TestBound> = IntervalSet::new(vec![interval]);
    let mut iter = set.iter();
    
    assert_eq!(iter.0.len(), 1);
    assert_eq!(iter.0.next(), Some(&interval));
}

#[test]
fn test_iter_multiple() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);

    impl TestBound {
        fn increment(&self) -> TestBound {
            TestBound(self.0 + 1)
        }
        fn decrement(&self) -> TestBound {
            TestBound(self.0 - 1)
        }
    }

    impl Interval for TestBound {
        type Bound = Self;

        fn lower(&self) -> Self::Bound { *self }
        fn upper(&self) -> Self::Bound { *self }
        fn set_lower(&mut self, bound: Self::Bound) { *self = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { *self = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let intervals = vec![TestBound(1), TestBound(2), TestBound(3)];
    let set: IntervalSet<TestBound> = IntervalSet::new(intervals.clone());
    let iter = set.iter();
    
    let collected: Vec<_> = iter.0.collect();
    assert_eq!(collected, intervals);
}

