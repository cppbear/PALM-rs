// Answer 0

#[test]
fn test_push_interval() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleBound(u32);
    
    impl SimpleBound {
        fn increment(&self) -> Self {
            SimpleBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            SimpleBound(self.0 - 1)
        }
    }

    impl Debug for SimpleBound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct SimpleInterval {
        lower: SimpleBound,
        upper: SimpleBound,
    }

    impl Interval for SimpleInterval {
        type Bound = SimpleBound;

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

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut interval_set = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(1),
            upper: SimpleBound(3),
        },
        SimpleInterval {
            lower: SimpleBound(5),
            upper: SimpleBound(8),
        }
    ]);

    interval_set.push(SimpleInterval {
        lower: SimpleBound(2),
        upper: SimpleBound(6),
    });

    let intervals = interval_set.intervals();
    assert_eq!(intervals.len(), 2);
    assert_eq!(intervals[0], SimpleInterval {
        lower: SimpleBound(1),
        upper: SimpleBound(6),
    });
    assert_eq!(intervals[1], SimpleInterval {
        lower: SimpleBound(2),
        upper: SimpleBound(5),
    });
}

#[test]
fn test_push_disjoint_intervals() {
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
    struct SimpleBound(u32);
    
    impl SimpleBound {
        fn increment(&self) -> Self {
            SimpleBound(self.0 + 1)
        }
        fn decrement(&self) -> Self {
            SimpleBound(self.0 - 1)
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct SimpleInterval {
        lower: SimpleBound,
        upper: SimpleBound,
    }

    impl Interval for SimpleInterval {
        type Bound = SimpleBound;

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

        fn case_fold_simple(&self, _intervals: &mut Vec<Self>) {}

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper >= other.lower
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let mut interval_set = IntervalSet::new(vec![
        SimpleInterval {
            lower: SimpleBound(1),
            upper: SimpleBound(2),
        },
        SimpleInterval {
            lower: SimpleBound(4),
            upper: SimpleBound(5),
        }
    ]);

    interval_set.push(SimpleInterval {
        lower: SimpleBound(6),
        upper: SimpleBound(7),
    });

    let intervals = interval_set.intervals();
    assert_eq!(intervals.len(), 3);
    assert_eq!(intervals[0], SimpleInterval {
        lower: SimpleBound(1),
        upper: SimpleBound(2),
    });
    assert_eq!(intervals[1], SimpleInterval {
        lower: SimpleBound(4),
        upper: SimpleBound(5),
    });
    assert_eq!(intervals[2], SimpleInterval {
        lower: SimpleBound(6),
        upper: SimpleBound(7),
    });
}

