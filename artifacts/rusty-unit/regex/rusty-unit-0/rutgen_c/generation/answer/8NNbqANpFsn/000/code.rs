// Answer 0

#[test]
fn test_new_empty_intervals() {
    struct TestInterval {
        lower: i32,
        upper: i32,
    }

    impl Clone for TestInterval {
        fn clone(&self) -> Self {
            TestInterval {
                lower: self.lower,
                upper: self.upper,
            }
        }
    }

    impl Copy for TestInterval {}

    impl Debug for TestInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "TestInterval({}, {})", self.lower, self.upper)
        }
    }

    impl Default for TestInterval {
        fn default() -> Self {
            TestInterval { lower: 0, upper: 0 }
        }
    }

    impl Eq for TestInterval {}

    impl PartialEq for TestInterval {
        fn eq(&self, other: &Self) -> bool {
            self.lower == other.lower && self.upper == other.upper
        }
    }

    impl PartialOrd for TestInterval {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.lower.partial_cmp(&other.lower)
        }
    }

    impl Ord for TestInterval {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.lower.cmp(&other.lower)
        }
    }

    impl Interval for TestInterval {
        type Bound = i32;

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

        fn is_contiguous(&self, _: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _: &Self) -> bool {
            false
        }

        fn is_subset(&self, _: &Self) -> bool {
            false
        }
    }

    let intervals: Vec<TestInterval> = vec![];
    let set = IntervalSet::new(intervals);
    assert_eq!(set.ranges.len(), 0);
}

#[test]
fn test_new_single_interval() {
    struct TestInterval {
        lower: i32,
        upper: i32,
    }

    impl Clone for TestInterval {
        fn clone(&self) -> Self {
            TestInterval {
                lower: self.lower,
                upper: self.upper,
            }
        }
    }

    impl Copy for TestInterval {}

    impl Debug for TestInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "TestInterval({}, {})", self.lower, self.upper)
        }
    }

    impl Default for TestInterval {
        fn default() -> Self {
            TestInterval { lower: 0, upper: 0 }
        }
    }

    impl Eq for TestInterval {}

    impl PartialEq for TestInterval {
        fn eq(&self, other: &Self) -> bool {
            self.lower == other.lower && self.upper == other.upper
        }
    }

    impl PartialOrd for TestInterval {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.lower.partial_cmp(&other.lower)
        }
    }

    impl Ord for TestInterval {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.lower.cmp(&other.lower)
        }
    }

    impl Interval for TestInterval {
        type Bound = i32;

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

        fn is_contiguous(&self, _: &Self) -> bool {
            false
        }

        fn is_intersection_empty(&self, _: &Self) -> bool {
            false
        }

        fn is_subset(&self, _: &Self) -> bool {
            false
        }
    }

    let intervals = vec![TestInterval { lower: 1, upper: 5 }];
    let set = IntervalSet::new(intervals);
    assert_eq!(set.ranges.len(), 1);
    assert_eq!(set.ranges[0], TestInterval { lower: 1, upper: 5 });
}

#[test]
fn test_new_overlapping_intervals() {
    struct TestInterval {
        lower: i32,
        upper: i32,
    }

    impl Clone for TestInterval {
        fn clone(&self) -> Self {
            TestInterval {
                lower: self.lower,
                upper: self.upper,
            }
        }
    }

    impl Copy for TestInterval {}

    impl Debug for TestInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "TestInterval({}, {})", self.lower, self.upper)
        }
    }

    impl Default for TestInterval {
        fn default() -> Self {
            TestInterval { lower: 0, upper: 0 }
        }
    }

    impl Eq for TestInterval {}

    impl PartialEq for TestInterval {
        fn eq(&self, other: &Self) -> bool {
            self.lower == other.lower && self.upper == other.upper
        }
    }

    impl PartialOrd for TestInterval {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.lower.partial_cmp(&other.lower)
        }
    }

    impl Ord for TestInterval {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.lower.cmp(&other.lower)
        }
    }

    impl Interval for TestInterval {
        type Bound = i32;

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
            self.lower <= other.upper && other.lower <= self.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn is_subset(&self, other: &Self) -> bool {
            self.lower >= other.lower && self.upper <= other.upper
        }
    }

    let intervals = vec![TestInterval { lower: 1, upper: 5 }, TestInterval { lower: 4, upper: 6 }];
    let set = IntervalSet::new(intervals);
    assert_eq!(set.ranges.len(), 1);
    assert_eq!(set.ranges[0], TestInterval { lower: 1, upper: 6 });
}

