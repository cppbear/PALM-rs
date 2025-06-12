// Answer 0

#[test]
fn test_negate_with_empty_ranges() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);

    impl TestBound {
        fn min_value() -> Self { TestBound(0) }
        fn max_value() -> Self { TestBound(u8::MAX) }
        fn increment(self) -> Self { TestBound(self.0 + 1) }
        fn decrement(self) -> Self { TestBound(self.0 - 1) }
    }

    impl Interval for TestBound {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound { *self }
        fn upper(&self) -> Self::Bound { *self }
        fn set_lower(&mut self, _bound: Self::Bound) {}
        fn set_upper(&mut self, _bound: Self::Bound) {}
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![TestBound::min_value(), TestBound::max_value()]);
    interval_set.negate();
}

#[test]
fn test_negate_with_single_interval() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);

    impl TestBound {
        fn min_value() -> Self { TestBound(1) }
        fn max_value() -> Self { TestBound(u8::MAX) }
        fn increment(self) -> Self { TestBound(self.0 + 1) }
        fn decrement(self) -> Self { TestBound(self.0 - 1) }
    }

    impl Interval for TestBound {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound { *self }
        fn upper(&self) -> Self::Bound { *self }
        fn set_lower(&mut self, _bound: Self::Bound) {}
        fn set_upper(&mut self, _bound: Self::Bound) {}
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![TestBound(1)]);
    interval_set.negate();
}

#[test]
fn test_negate_with_adjacent_intervals() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);

    impl TestBound {
        fn min_value() -> Self { TestBound(1) }
        fn max_value() -> Self { TestBound(5) }
        fn increment(self) -> Self { TestBound(self.0 + 1) }
        fn decrement(self) -> Self { TestBound(self.0 - 1) }
    }

    impl Interval for TestBound {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound { *self }
        fn upper(&self) -> Self::Bound { *self }
        fn set_lower(&mut self, _bound: Self::Bound) {}
        fn set_upper(&mut self, _bound: Self::Bound) {}
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { false }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { false }
    }

    let mut interval_set = IntervalSet::new(vec![TestBound(1), TestBound(2)]);
    interval_set.negate();
}

#[test]
fn test_negate_with_complete_range() {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct TestBound(u8);

    impl TestBound {
        fn min_value() -> Self { TestBound(0) }
        fn max_value() -> Self { TestBound(u8::MAX) }
        fn increment(self) -> Self { TestBound(self.0 + 1) }
        fn decrement(self) -> Self { TestBound(self.0 - 1) }
    }

    impl Interval for TestBound {
        type Bound = TestBound;

        fn lower(&self) -> Self::Bound { *self }
        fn upper(&self) -> Self::Bound { *self }
        fn set_lower(&mut self, _bound: Self::Bound) {}
        fn set_upper(&mut self, _bound: Self::Bound) {}
        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}
        fn is_contiguous(&self, _other: &Self) -> bool { true }
        fn is_intersection_empty(&self, _other: &Self) -> bool { false }
        fn is_subset(&self, _other: &Self) -> bool { true }
    }

    let mut interval_set = IntervalSet::new(vec![TestBound::min_value(), TestBound::max_value()]);
    interval_set.negate();
}

