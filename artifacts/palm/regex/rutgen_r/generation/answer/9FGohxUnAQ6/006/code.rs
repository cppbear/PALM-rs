// Answer 0

#[derive(Debug)]
struct Range {
    lower: i32,
    upper: i32,
}

impl Range {
    fn is_intersection_empty(&self, other: &Range) -> bool {
        self.upper < other.lower || self.lower > other.upper
    }

    fn lower(&self) -> i32 {
        self.lower
    }

    fn upper(&self) -> i32 {
        self.upper
    }

    fn difference(&self, other: &Range) -> (Option<Range>, Option<Range>) {
        if self.is_intersection_empty(other) {
            return (Some(*self), None);
        }

        if self.lower < other.lower {
            if self.upper > other.upper {
                return (Some(Range { lower: self.lower, upper: other.lower - 1 }), Some(Range { lower: other.upper + 1, upper: self.upper }));
            } else {
                return (Some(Range { lower: self.lower, upper: other.lower - 1 }), None);
            }
        } else if self.upper > other.upper {
            return (None, Some(Range { lower: other.upper + 1, upper: self.upper }));
        } else {
            return (None, None);
        }
    }
}

#[derive(Debug)]
struct IntervalSet {
    ranges: Vec<Range>,
}

impl IntervalSet {
    pub fn new() -> Self {
        IntervalSet { ranges: Vec::new() }
    }

    pub fn difference(&mut self, other: &IntervalSet) {
        // The original difference function implementation 
        // goes here (omitted for brevity).
    }
}

#[test]
fn test_difference_no_overlap() {
    let mut set_a = IntervalSet {
        ranges: vec![Range { lower: 5, upper: 10 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Range { lower: 11, upper: 15 }],
    };

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0], Range { lower: 5, upper: 10 });
}

#[test]
fn test_difference_full_overlap() {
    let mut set_a = IntervalSet {
        ranges: vec![Range { lower: 2, upper: 5 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Range { lower: 2, upper: 5 }],
    };

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 0);
}

#[test]
fn test_difference_partial_overlap() {
    let mut set_a = IntervalSet {
        ranges: vec![Range { lower: 2, upper: 5 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Range { lower: 3, upper: 4 }],
    };

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0], Range { lower: 2, upper: 2 });
}

#[test]
fn test_difference_with_multiple_ranges() {
    let mut set_a = IntervalSet {
        ranges: vec![Range { lower: 1, upper: 10 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Range { lower: 5, upper: 7 }, Range { lower: 8, upper: 12 }],
    };

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 2);
    assert_eq!(set_a.ranges[0], Range { lower: 1, upper: 4 });
    assert_eq!(set_a.ranges[1], Range { lower: 8, upper: 10 });
}

#[test]
fn test_difference_multiple_removals() {
    let mut set_a = IntervalSet {
        ranges: vec![Range { lower: 1, upper: 10 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Range { lower: 3, upper: 5 }, Range { lower: 7, upper: 9 }],
    };

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0], Range { lower: 1, upper: 2 });
}

