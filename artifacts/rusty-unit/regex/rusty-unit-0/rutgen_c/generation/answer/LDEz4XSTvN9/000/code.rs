// Answer 0

#[test]
fn test_negate_empty_set() {
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct Bound(u8);
    
    impl Bound {
        fn min_value() -> Self {
            Bound(0)
        }
        
        fn max_value() -> Self {
            Bound(u8::MAX)
        }
        
        fn decrement(self) -> Self {
            Bound(self.0.saturating_sub(1))
        }
        
        fn increment(self) -> Self {
            Bound(self.0.saturating_add(1))
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct ExampleInterval {
        lower: Bound,
        upper: Bound,
    }
    
    impl Interval for ExampleInterval {
        type Bound = Bound;

        fn lower(&self) -> Self::Bound {
            self.lower.clone()
        }

        fn upper(&self) -> Self::Bound {
            self.upper.clone()
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool {
            true
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut interval_set: IntervalSet<ExampleInterval> = IntervalSet::new(vec![]);
    interval_set.negate();
    assert_eq!(interval_set.ranges.len(), 1);
    assert_eq!(interval_set.ranges[0], ExampleInterval { lower: Bound::min_value(), upper: Bound::max_value() });
}

#[test]
fn test_negate_non_empty_set() {
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct Bound(u8);
    
    impl Bound {
        fn min_value() -> Self {
            Bound(0)
        }
        
        fn max_value() -> Self {
            Bound(u8::MAX)
        }
        
        fn decrement(self) -> Self {
            Bound(self.0.saturating_sub(1))
        }
        
        fn increment(self) -> Self {
            Bound(self.0.saturating_add(1))
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct ExampleInterval {
        lower: Bound,
        upper: Bound,
    }
    
    impl Interval for ExampleInterval {
        type Bound = Bound;

        fn lower(&self) -> Self::Bound {
            self.lower.clone()
        }

        fn upper(&self) -> Self::Bound {
            self.upper.clone()
        }

        fn set_lower(&mut self, bound: Self::Bound) {
            self.lower = bound;
        }

        fn set_upper(&mut self, bound: Self::Bound) {
            self.upper = bound;
        }

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, _other: &Self) -> bool {
            true
        }

        fn is_intersection_empty(&self, _other: &Self) -> bool {
            false
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut interval_set: IntervalSet<ExampleInterval> = IntervalSet::new(vec![
        ExampleInterval { lower: Bound(5), upper: Bound(10) },
        ExampleInterval { lower: Bound(15), upper: Bound(20) },
    ]);
    
    interval_set.negate();
    assert_eq!(interval_set.ranges.len(), 3);
    assert_eq!(interval_set.ranges[0], ExampleInterval { lower: Bound::min_value(), upper: Bound(4) });
    assert_eq!(interval_set.ranges[1], ExampleInterval { lower: Bound(11), upper: Bound(14) });
    assert_eq!(interval_set.ranges[2], ExampleInterval { lower: Bound(21), upper: Bound::max_value() });
}

