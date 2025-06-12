// Answer 0

#[test]
fn test_iter_empty() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct MockBound(u8);

    impl Bound for MockBound {
        fn increment(&self) -> Self {
            MockBound(self.0 + 1)
        }
        
        fn decrement(&self) -> Self {
            MockBound(self.0.saturating_sub(1))
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MockInterval {
        lower: MockBound,
        upper: MockBound,
    }

    impl Interval for MockInterval {
        type Bound = MockBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let interval_set: IntervalSet<MockInterval> = IntervalSet::new(vec![]);
    let mut iter = interval_set.iter();
    assert_eq!(iter.0.next(), None);
}

#[test]
fn test_iter_single() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct MockBound(u8);

    impl Bound for MockBound {
        fn increment(&self) -> Self {
            MockBound(self.0 + 1)
        }
        
        fn decrement(&self) -> Self {
            MockBound(self.0.saturating_sub(1))
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MockInterval {
        lower: MockBound,
        upper: MockBound,
    }

    impl Interval for MockInterval {
        type Bound = MockBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let interval_set: IntervalSet<MockInterval> = IntervalSet::new(vec![MockInterval { lower: MockBound(1), upper: MockBound(2) }]);
    let mut iter = interval_set.iter();
    assert_eq!(iter.0.next(), Some(&MockInterval { lower: MockBound(1), upper: MockBound(2) }));
    assert_eq!(iter.0.next(), None);
}

#[test]
fn test_iter_multiple() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct MockBound(u8);

    impl Bound for MockBound {
        fn increment(&self) -> Self {
            MockBound(self.0 + 1)
        }
        
        fn decrement(&self) -> Self {
            MockBound(self.0.saturating_sub(1))
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MockInterval {
        lower: MockBound,
        upper: MockBound,
    }

    impl Interval for MockInterval {
        type Bound = MockBound;

        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound; }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound; }
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let interval_set: IntervalSet<MockInterval> = IntervalSet::new(vec![
        MockInterval { lower: MockBound(1), upper: MockBound(2) },
        MockInterval { lower: MockBound(3), upper: MockBound(4) },
    ]);

    let mut iter = interval_set.iter();
    assert_eq!(iter.0.next(), Some(&MockInterval { lower: MockBound(1), upper: MockBound(2) }));
    assert_eq!(iter.0.next(), Some(&MockInterval { lower: MockBound(3), upper: MockBound(4) }));
    assert_eq!(iter.0.next(), None);
}

