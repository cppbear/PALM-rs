// Answer 0

#[test]
fn test_difference_non_empty_self_empty_other() {
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
            if self.is_intersection_empty(&other) {
                return (Some(self.clone()), None);
            }
            if self.lower <= other.lower && self.upper >= other.upper {
                return (None, None);
            }
            if self.lower < other.lower {
                return (Some(Interval { lower: self.lower, upper: other.lower - 1 }), 
                        Some(Interval { lower: other.upper + 1, upper: self.upper }));
            }
            if self.upper > other.upper {
                return (Some(Interval { lower: other.upper + 1, upper: self.upper }), None);
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

        fn push(&mut self, range: Interval) {
            self.ranges.push(range);
        }
    }

    let mut set = IntervalSet::new();
    set.push(Interval { lower: 1, upper: 5 });
    let other = IntervalSet::new(); // empty

    set.difference(&other);

    assert_eq!(set.ranges.len(), 1);
    assert_eq!(set.ranges[0].lower, 1);
    assert_eq!(set.ranges[0].upper, 5);
}

#[test]
#[should_panic]
fn test_difference_should_panic_on_empty_other() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl IntervalSet {
        fn new() -> Self {
            Self { ranges: Vec::new() }
        }

        pub fn difference(&mut self, other: &IntervalSet) {
            if self.ranges.is_empty() || other.ranges.is_empty() {
                panic!("One of the interval sets is empty");
            }
        }
    }

    let mut set = IntervalSet::new();
    set.push(Interval { lower: 1, upper: 5 });
    let other = IntervalSet::new(); // empty

    set.difference(&other); // this should panic
}

