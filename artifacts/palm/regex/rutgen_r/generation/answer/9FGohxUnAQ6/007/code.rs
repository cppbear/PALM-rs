// Answer 0

#[derive(Debug, Clone)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn upper(&self) -> i32 {
        self.end
    }

    fn lower(&self) -> i32 {
        self.start
    }

    fn is_intersection_empty(&self, other: &Interval) -> bool {
        self.upper() < other.lower() || other.upper() < self.lower()
    }

    fn difference(&self, other: &Interval) -> (Option<Interval>, Option<Interval>) {
        if self.is_intersection_empty(other) {
            return (Some(self.clone()), None);
        }
        if self.lower() < other.lower() && self.upper() > other.upper() {
            return (Some(Interval { start: self.lower(), end: other.lower() }), Some(Interval { start: other.upper(), end: self.upper() }));
        }
        if self.lower() < other.lower() {
            return (Some(Interval { start: self.lower(), end: other.lower() }), None);
        }
        if self.upper() > other.upper() {
            return (None, Some(Interval { start: other.upper(), end: self.upper() }));
        }
        (None, None)
    }
}

#[derive(Debug, Default)]
struct IntervalSet {
    ranges: Vec<Interval>,
}

impl IntervalSet {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    pub fn difference(&mut self, other: &IntervalSet) {
        if self.ranges.is_empty() || other.ranges.is_empty() {
            return;
        }
        // Implementation of the function goes here (as provided)
    }
}

#[test]
fn test_difference_non_intersecting_ranges() {
    let mut set_a = IntervalSet {
        ranges: vec![Interval { start: 5, end: 10 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Interval { start: 0, end: 4 }],
    };
    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].start, 5);
    assert_eq!(set_a.ranges[0].end, 10);
}

#[test]
fn test_difference_with_overlap() {
    let mut set_a = IntervalSet {
        ranges: vec![Interval { start: 5, end: 10 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Interval { start: 7, end: 8 }],
    };
    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].start, 5);
    assert_eq!(set_a.ranges[0].end, 7);
}

#[test]
fn test_difference_full_overlap() {
    let mut set_a = IntervalSet {
        ranges: vec![Interval { start: 5, end: 10 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Interval { start: 5, end: 10 }],
    };
    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 0);
}

#[test]
fn test_difference_adjacent_ranges() {
    let mut set_a = IntervalSet {
        ranges: vec![Interval { start: 5, end: 10 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Interval { start: 10, end: 15 }],
    };
    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0].start, 5);
    assert_eq!(set_a.ranges[0].end, 10);
}

#[test]
fn test_difference_multiple_ranges() {
    let mut set_a = IntervalSet {
        ranges: vec![Interval { start: 1, end: 5 }, Interval { start: 10, end: 15 }],
    };
    let set_b = IntervalSet {
        ranges: vec![Interval { start: 3, end: 12 }],
    };
    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 2);
    assert_eq!(set_a.ranges[0].start, 1);
    assert_eq!(set_a.ranges[0].end, 3);
    assert_eq!(set_a.ranges[1].start, 12);
    assert_eq!(set_a.ranges[1].end, 15);
}

