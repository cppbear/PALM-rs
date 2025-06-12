// Answer 0

#[test]
fn test_intersect_non_empty() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    impl Interval {
        fn intersect(&self, other: &Self) -> Option<Self> {
            if self.lower > other.upper || self.upper < other.lower {
                None
            } else {
                Some(Self {
                    lower: self.lower.max(other.lower),
                    upper: self.upper.min(other.upper),
                })
            }
        }

        fn upper(&self) -> i32 {
            self.upper
        }
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    let mut set_a = IntervalSet {
        ranges: vec![
            Interval { lower: 1, upper: 5 },
            Interval { lower: 6, upper: 10 },
        ],
    };

    let set_b = IntervalSet {
        ranges: vec![
            Interval { lower: 3, upper: 7 },
            Interval { lower: 8, upper: 12 },
        ],
    };

    set_a.intersect(&set_b);

    assert_eq!(set_a.ranges.len(), 3);
    assert_eq!(set_a.ranges[0].lower, 3);
    assert_eq!(set_a.ranges[0].upper, 5);
    assert_eq!(set_a.ranges[1].lower, 6);
    assert_eq!(set_a.ranges[1].upper, 7);
    assert_eq!(set_a.ranges[2].lower, 8);
    assert_eq!(set_a.ranges[2].upper, 10);
}

#[test]
#[should_panic]
fn test_intersect_empty_self_ranges() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    impl Interval {
        fn intersect(&self, other: &Self) -> Option<Self> {
            if self.lower > other.upper || self.upper < other.lower {
                None
            } else {
                Some(Self {
                    lower: self.lower.max(other.lower),
                    upper: self.upper.min(other.upper),
                })
            }
        }

        fn upper(&self) -> i32 {
            self.upper
        }
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    let mut set_a = IntervalSet { ranges: vec![] };
    
    let set_b = IntervalSet {
        ranges: vec![Interval { lower: 1, upper: 5 }],
    };

    set_a.intersect(&set_b);
}

#[test]
#[should_panic]
fn test_intersect_empty_other_ranges() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    impl Interval {
        fn intersect(&self, other: &Self) -> Option<Self> {
            if self.lower > other.upper || self.upper < other.lower {
                None
            } else {
                Some(Self {
                    lower: self.lower.max(other.lower),
                    upper: self.upper.min(other.upper),
                })
            }
        }

        fn upper(&self) -> i32 {
            self.upper
        }
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    let mut set_a = IntervalSet {
        ranges: vec![Interval { lower: 1, upper: 5 }],
    };
    
    let set_b = IntervalSet { ranges: vec![] };

    set_a.intersect(&set_b);
}

#[test]
#[should_panic]
fn test_intersect_no_valid_intersection() {
    struct Interval {
        lower: i32,
        upper: i32,
    }

    impl Interval {
        fn intersect(&self, other: &Self) -> Option<Self> {
            if self.lower > other.upper || self.upper < other.lower {
                None
            } else {
                Some(Self {
                    lower: self.lower.max(other.lower),
                    upper: self.upper.min(other.upper),
                })
            }
        }

        fn upper(&self) -> i32 {
            self.upper
        }
    }

    struct IntervalSet<I> {
        ranges: Vec<I>,
    }

    let mut set_a = IntervalSet {
        ranges: vec![Interval { lower: 1, upper: 2 }],
    };

    let set_b = IntervalSet {
        ranges: vec![Interval { lower: 3, upper: 4 }],
    };

    set_a.intersect(&set_b);
}

