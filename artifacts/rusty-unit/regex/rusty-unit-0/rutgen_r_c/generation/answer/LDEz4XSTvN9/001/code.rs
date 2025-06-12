// Answer 0

#[test]
fn test_negate_empty_interval_set() {
    use std::cmp::Ordering;

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct Bound {
        value: i32,
    }

    impl Bound {
        fn min_value() -> Self {
            Bound { value: i32::MIN }
        }

        fn max_value() -> Self {
            Bound { value: i32::MAX }
        }
        
        fn increment(&self) -> Self {
            Bound { value: self.value + 1 }
        }

        fn decrement(&self) -> Self {
            Bound { value: self.value - 1 }
        }
    }

    impl Debug for Bound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestInterval {
        lower: Bound,
        upper: Bound,
    }

    impl Interval for TestInterval {
        type Bound = Bound;

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

        fn is_contiguous(&self, _other: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut interval_set: IntervalSet<TestInterval> = IntervalSet::new(Vec::<TestInterval>::new());
    interval_set.negate();

    assert_eq!(interval_set.ranges.len(), 1);
    assert_eq!(interval_set.ranges[0].lower(), Bound::min_value());
    assert_eq!(interval_set.ranges[0].upper(), Bound::max_value());
}

