// Answer 0

#[test]
fn test_intervals_empty() {
    #[derive(Clone, Debug, Eq, PartialEq, Default)]
    struct DummyInterval;

    impl Interval for DummyInterval {
        type Bound = u8;
        fn lower(&self) -> Self::Bound { 0 }
        fn upper(&self) -> Self::Bound { 0 }
        fn set_lower(&mut self, _: Self::Bound) {}
        fn set_upper(&mut self, _: Self::Bound) {}
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let set: IntervalSet<DummyInterval> = IntervalSet::new(vec![]);
    let result = set.intervals();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_intervals_single() {
    #[derive(Clone, Debug, Eq, PartialEq, Default)]
    struct DummyInterval {
        lower: u8,
        upper: u8,
    }

    impl Interval for DummyInterval {
        type Bound = u8;
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let set: IntervalSet<DummyInterval> = IntervalSet::new(vec![DummyInterval { lower: 1, upper: 5 }]);
    let result = set.intervals();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], DummyInterval { lower: 1, upper: 5 });
}

#[test]
fn test_intervals_multiple() {
    #[derive(Clone, Debug, Eq, PartialEq, Default)]
    struct DummyInterval {
        lower: u8,
        upper: u8,
    }

    impl Interval for DummyInterval {
        type Bound = u8;
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let set: IntervalSet<DummyInterval> = IntervalSet::new(vec![
        DummyInterval { lower: 1, upper: 3 },
        DummyInterval { lower: 4, upper: 7 },
        DummyInterval { lower: 8, upper: 10 },
    ]);
    let result = set.intervals();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], DummyInterval { lower: 1, upper: 3 });
    assert_eq!(result[1], DummyInterval { lower: 4, upper: 7 });
    assert_eq!(result[2], DummyInterval { lower: 8, upper: 10 });
}

#[test]
fn test_intervals_canonical_ordering() {
    #[derive(Clone, Debug, Eq, PartialEq, Default)]
    struct DummyInterval {
        lower: u8,
        upper: u8,
    }

    impl Interval for DummyInterval {
        type Bound = u8;
        fn lower(&self) -> Self::Bound { self.lower }
        fn upper(&self) -> Self::Bound { self.upper }
        fn set_lower(&mut self, bound: Self::Bound) { self.lower = bound }
        fn set_upper(&mut self, bound: Self::Bound) { self.upper = bound }
        fn case_fold_simple(&self, _: &mut Vec<Self>) {}
        fn is_contiguous(&self, _: &Self) -> bool { true }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn is_subset(&self, _: &Self) -> bool { false }
    }

    let set: IntervalSet<DummyInterval> = IntervalSet::new(vec![
        DummyInterval { lower: 5, upper: 7 },
        DummyInterval { lower: 1, upper: 3 },
        DummyInterval { lower: 4, upper: 6 },
    ]);
    let result = set.intervals();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], DummyInterval { lower: 5, upper: 7 });
    assert_eq!(result[1], DummyInterval { lower: 1, upper: 3 });
    assert_eq!(result[2], DummyInterval { lower: 4, upper: 6 });
}

