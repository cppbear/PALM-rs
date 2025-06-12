// Answer 0

#[test]
fn test_case_fold_simple_with_single_range() {
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct CharInterval {
        lower: char,
        upper: char,
    }

    impl Interval for CharInterval {
        type Bound = char;

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

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {
            intervals.push(CharInterval {
                lower: self.lower.to_ascii_lowercase(),
                upper: self.upper.to_ascii_uppercase(),
            });
        }

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

    let mut intervals = IntervalSet::new(vec![CharInterval { lower: 'a', upper: 'z' }]);
    intervals.case_fold_simple();
    assert_eq!(intervals.intervals(), &[CharInterval { lower: 'a', upper: 'z' }, CharInterval { lower: 'A', upper: 'Z' }]);
}

#[test]
fn test_case_fold_simple_with_non_contiguous_ranges() {
    #[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
    struct CharInterval {
        lower: char,
        upper: char,
    }

    impl Interval for CharInterval {
        type Bound = char;

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

        fn case_fold_simple(&self, intervals: &mut Vec<Self>) {
            intervals.push(CharInterval {
                lower: self.lower.to_ascii_lowercase(),
                upper: self.upper.to_ascii_uppercase(),
            });
        }

        fn is_contiguous(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_intersection_empty(&self, other: &Self) -> bool {
            self.upper < other.lower || self.lower > other.upper
        }

        fn is_subset(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut intervals = IntervalSet::new(vec![CharInterval { lower: 'A', upper: 'A' }, CharInterval { lower: 'b', upper: 'b' }]);
    intervals.case_fold_simple();
    assert_eq!(intervals.intervals(), &[CharInterval { lower: 'A', upper: 'A' }, CharInterval { lower: 'a', upper: 'a' }, CharInterval { lower: 'B', upper: 'B' }, CharInterval { lower: 'b', upper: 'b' }]);
}

