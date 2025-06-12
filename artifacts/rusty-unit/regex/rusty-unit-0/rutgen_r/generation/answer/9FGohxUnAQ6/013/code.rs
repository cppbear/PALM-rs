// Answer 0

#[test]
fn test_difference_non_overlapping_ranges() {
    struct Interval {
        lower: usize,
        upper: usize,
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl Interval {
        fn lower(&self) -> usize { self.lower }
        fn upper(&self) -> usize { self.upper }
        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }
        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(self.clone()), None);
            }
            let mut ranges = Vec::new();
            if self.lower < other.lower {
                ranges.push(Interval { lower: self.lower, upper: other.lower });
            }
            if self.upper > other.upper {
                ranges.push(Interval { lower: other.upper, upper: self.upper });
            }
            if ranges.is_empty() {
                return (None, None);
            }
            (Some(ranges[0].clone()), if ranges.len() > 1 { Some(ranges[1].clone()) } else { None })
        }
    }

    impl Clone for Interval {
        fn clone(&self) -> Self {
            Interval { lower: self.lower, upper: self.upper }
        }
    }

    let mut set_a = IntervalSet { ranges: vec![Interval { lower: 1, upper: 5 }] };
    let set_b = IntervalSet { ranges: vec![Interval { lower: 6, upper: 10 }] };

    set_a.difference(&set_b);

    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].lower, 1);
    assert_eq!(set_a.ranges[0].upper, 5);
}

#[test]
fn test_difference_with_partial_overlap() {
    struct Interval {
        lower: usize,
        upper: usize,
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl Interval {
        fn lower(&self) -> usize { self.lower }
        fn upper(&self) -> usize { self.upper }
        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }
        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(self.clone()), None);
            }
            let mut ranges = Vec::new();
            if self.lower < other.lower {
                ranges.push(Interval { lower: self.lower, upper: other.lower });
            }
            if self.upper > other.upper {
                ranges.push(Interval { lower: other.upper, upper: self.upper });
            }
            if ranges.is_empty() {
                return (None, None);
            }
            (Some(ranges[0].clone()), if ranges.len() > 1 { Some(ranges[1].clone()) } else { None })
        }
    }

    impl Clone for Interval {
        fn clone(&self) -> Self {
            Interval { lower: self.lower, upper: self.upper }
        }
    }

    let mut set_a = IntervalSet { ranges: vec![Interval { lower: 1, upper: 5 }] };
    let set_b = IntervalSet { ranges: vec![Interval { lower: 3, upper: 4 }] };

    set_a.difference(&set_b);

    assert_eq!(set_a.ranges.len(), 2);
    assert_eq!(set_a.ranges[0].lower, 1);
    assert_eq!(set_a.ranges[0].upper, 3);
    assert_eq!(set_a.ranges[1].lower, 4);
    assert_eq!(set_a.ranges[1].upper, 5);
}

#[test]
fn test_difference_completely_overlapping() {
    struct Interval {
        lower: usize,
        upper: usize,
    }

    struct IntervalSet {
        ranges: Vec<Interval>,
    }

    impl Interval {
        fn lower(&self) -> usize { self.lower }
        fn upper(&self) -> usize { self.upper }
        fn is_intersection_empty(&self, other: &Interval) -> bool {
            self.upper < other.lower || other.upper < self.lower
        }
        fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
            if self.is_intersection_empty(other) {
                return (Some(self.clone()), None);
            }
            let mut ranges = Vec::new();
            if self.lower < other.lower {
                ranges.push(Interval { lower: self.lower, upper: other.lower });
            }
            if self.upper > other.upper {
                ranges.push(Interval { lower: other.upper, upper: self.upper });
            }
            if ranges.is_empty() {
                return (None, None);
            }
            (Some(ranges[0].clone()), if ranges.len() > 1 { Some(ranges[1].clone()) } else { None })
        }
    }

    impl Clone for Interval {
        fn clone(&self) -> Self {
            Interval { lower: self.lower, upper: self.upper }
        }
    }

    let mut set_a = IntervalSet { ranges: vec![Interval { lower: 1, upper: 5 }] };
    let set_b = IntervalSet { ranges: vec![Interval { lower: 1, upper: 5 }] };

    set_a.difference(&set_b);

    assert_eq!(set_a.ranges.len(), 0);
}

