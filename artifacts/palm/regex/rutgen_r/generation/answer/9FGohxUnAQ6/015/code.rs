// Answer 0

#[test]
fn test_difference_no_overlap() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    impl Interval {
        fn lower(&self) -> i32 {
            self.lower
        }

        fn upper(&self) -> i32 {
            self.upper
        }

        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            } else if self.lower == other.lower && self.upper == other.upper {
                return (None, None);
            } else if self.lower < other.lower && self.upper > other.upper {
                return (Some(Interval { lower: self.lower, upper: other.lower }), Some(Interval { lower: other.upper, upper: self.upper }));
            } else if self.lower < other.lower {
                return (Some(Interval { lower: self.lower, upper: other.lower }), None);
            } else if self.upper > other.upper {
                return (None, Some(Interval { lower: other.upper, upper: self.upper }));
            }
            (None, None)
        }
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl IntervalSet {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        fn difference(&mut self, other: &IntervalSet) {
            // Function implementation provided in the context.
        }
    }

    let mut set1 = IntervalSet::new();
    let ranges1 = vec![Interval { lower: 1, upper: 5 }, Interval { lower: 6, upper: 10 }];
    set1.ranges.extend(ranges1);

    let mut set2 = IntervalSet::new();
    let ranges2 = vec![Interval { lower: 11, upper: 15 }, Interval { lower: 16, upper: 20 }];
    set2.ranges.extend(ranges2);

    set1.difference(&set2);
    assert_eq!(set1.ranges.len(), 2);
}

#[test]
fn test_difference_partial_overlap() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    impl Interval {
        fn lower(&self) -> i32 {
            self.lower
        }

        fn upper(&self) -> i32 {
            self.upper
        }

        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            } else if self.lower == other.lower && self.upper == other.upper {
                return (None, None);
            } else if self.lower < other.lower && self.upper > other.upper {
                return (Some(Interval { lower: self.lower, upper: other.lower }), Some(Interval { lower: other.upper, upper: self.upper }));
            } else if self.lower < other.lower {
                return (Some(Interval { lower: self.lower, upper: other.lower }), None);
            } else if self.upper > other.upper {
                return (None, Some(Interval { lower: other.upper, upper: self.upper }));
            }
            (None, None)
        }
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl IntervalSet {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        fn difference(&mut self, other: &IntervalSet) {
            // Function implementation provided in the context.
        }
    }

    let mut set1 = IntervalSet::new();
    let ranges1 = vec![Interval { lower: 1, upper: 5 }, Interval { lower: 6, upper: 10 }];
    set1.ranges.extend(ranges1);

    let mut set2 = IntervalSet::new();
    let ranges2 = vec![Interval { lower: 4, upper: 8 }];
    set2.ranges.extend(ranges2);

    set1.difference(&set2);
    assert_eq!(set1.ranges.len(), 2);
    assert_eq!(set1.ranges[0].lower, 1);
    assert_eq!(set1.ranges[0].upper, 4);
    assert_eq!(set1.ranges[1].lower, 8);
    assert_eq!(set1.ranges[1].upper, 10);
}

#[test]
fn test_difference_full_overlap() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    impl Interval {
        fn lower(&self) -> i32 {
            self.lower
        }

        fn upper(&self) -> i32 {
            self.upper
        }

        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            } else if self.lower == other.lower && self.upper == other.upper {
                return (None, None);
            } else if self.lower < other.lower && self.upper > other.upper {
                return (Some(Interval { lower: self.lower, upper: other.lower }), Some(Interval { lower: other.upper, upper: self.upper }));
            } else if self.lower < other.lower {
                return (Some(Interval { lower: self.lower, upper: other.lower }), None);
            } else if self.upper > other.upper {
                return (None, Some(Interval { lower: other.upper, upper: self.upper }));
            }
            (None, None)
        }
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl IntervalSet {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        fn difference(&mut self, other: &IntervalSet) {
            // Function implementation provided in the context.
        }
    }

    let mut set1 = IntervalSet::new();
    let ranges1 = vec![Interval { lower: 1, upper: 5 }];
    set1.ranges.extend(ranges1);

    let mut set2 = IntervalSet::new();
    let ranges2 = vec![Interval { lower: 1, upper: 5 }];
    set2.ranges.extend(ranges2);

    set1.difference(&set2);
    assert_eq!(set1.ranges.len(), 0);
}

#[test]
fn test_difference_multiple_ranges() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    impl Interval {
        fn lower(&self) -> i32 {
            self.lower
        }

        fn upper(&self) -> i32 {
            self.upper
        }

        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }

        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(*self), None);
            } else if self.lower == other.lower && self.upper == other.upper {
                return (None, None);
            } else if self.lower < other.lower && self.upper > other.upper {
                return (Some(Interval { lower: self.lower, upper: other.lower }), Some(Interval { lower: other.upper, upper: self.upper }));
            } else if self.lower < other.lower {
                return (Some(Interval { lower: self.lower, upper: other.lower }), None);
            } else if self.upper > other.upper {
                return (None, Some(Interval { lower: other.upper, upper: self.upper }));
            }
            (None, None)
        }
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl IntervalSet {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        fn difference(&mut self, other: &IntervalSet) {
            // Function implementation provided in the context.
        }
    }

    let mut set1 = IntervalSet::new();
    let ranges1 = vec![
        Interval { lower: 1, upper: 3 },
        Interval { lower: 4, upper: 6 },
        Interval { lower: 7, upper: 9 },
    ];
    set1.ranges.extend(ranges1);

    let mut set2 = IntervalSet::new();
    let ranges2 = vec![Interval { lower: 2, upper: 5 }];
    set2.ranges.extend(ranges2);

    set1.difference(&set2);
    assert_eq!(set1.ranges.len(), 3);
    assert_eq!(set1.ranges[0].lower, 1);
    assert_eq!(set1.ranges[0].upper, 2);
    assert_eq!(set1.ranges[1].lower, 5);
    assert_eq!(set1.ranges[1].upper, 6);
    assert_eq!(set1.ranges[2].lower, 7);
    assert_eq!(set1.ranges[2].upper, 9);
}

