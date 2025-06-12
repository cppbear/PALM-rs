// Answer 0

#[test]
fn test_symmetric_difference_no_overlap() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct MockBound(u8);

    impl std::ops::Add<u8> for MockBound {
        type Output = Self;

        fn add(self, rhs: u8) -> Self::Output {
            MockBound(self.0 + rhs)
        }
    }

    impl std::ops::Sub<u8> for MockBound {
        type Output = Self;

        fn sub(self, rhs: u8) -> Self::Output {
            MockBound(self.0 - rhs)
        }
    }

    impl std::ops::AddAssign<u8> for MockBound {
        fn add_assign(&mut self, rhs: u8) {
            self.0 += rhs;
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct MockInterval {
        lower: MockBound,
        upper: MockBound,
    }

    impl Interval for MockInterval {
        type Bound = MockBound;

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
            self.upper.0 < other.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.0 < other.lower.0 || other.upper.0 < self.lower.0
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
    }

    let mut set_a = IntervalSet::new(vec![MockInterval { lower: MockBound(1), upper: MockBound(3) }]);
    let set_b = IntervalSet::new(vec![MockInterval { lower: MockBound(4), upper: MockBound(6) }]);
    
    set_a.symmetric_difference(&set_b);
    
    let expected = vec![MockInterval { lower: MockBound(1), upper: MockBound(3) },
                        MockInterval { lower: MockBound(4), upper: MockBound(6) }];

    assert_eq!(set_a.intervals(), &expected);
}

#[test]
fn test_symmetric_difference_with_overlap() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct MockBound(u8);

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct MockInterval {
        lower: MockBound,
        upper: MockBound,
    }

    impl Interval for MockInterval {
        type Bound = MockBound;

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
            self.upper.0 >= other.lower.0 && self.lower.0 <= other.upper.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            other.lower.0 > self.upper.0 || self.lower.0 > other.upper.0
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
    }

    let mut set_a = IntervalSet::new(vec![MockInterval { lower: MockBound(1), upper: MockBound(5) }]);
    let set_b = IntervalSet::new(vec![MockInterval { lower: MockBound(3), upper: MockBound(7) }]);

    set_a.symmetric_difference(&set_b);

    let expected = vec![MockInterval { lower: MockBound(1), upper: MockBound(2) },
                        MockInterval { lower: MockBound(6), upper: MockBound(7) }];

    assert_eq!(set_a.intervals(), &expected);
}

#[test]
fn test_symmetric_difference_empty_set() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct MockBound(u8);

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct MockInterval {
        lower: MockBound,
        upper: MockBound,
    }

    impl Interval for MockInterval {
        type Bound = MockBound;

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
            self.upper.0 < other.lower.0
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper.0 < other.lower.0 || other.upper.0 < self.lower.0
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower.0 >= other.lower.0 && self.upper.0 <= other.upper.0
        }
    }

    let mut set_a = IntervalSet::new(vec![MockInterval { lower: MockBound(1), upper: MockBound(2) }]);
    let set_b = IntervalSet::new(Vec::<MockInterval>::new());

    set_a.symmetric_difference(&set_b);

    let expected = vec![MockInterval { lower: MockBound(1), upper: MockBound(2) }];

    assert_eq!(set_a.intervals(), &expected);
}

