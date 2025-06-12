// Answer 0

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn upper(&self) -> usize {
        self.end
    }

    fn lower(&self) -> usize {
        self.start
    }

    fn is_intersection_empty(&self, other: &Range) -> bool {
        self.upper() < other.lower() || other.upper() < self.lower()
    }

    fn difference(&self, other: &Range) -> (Option<Range>, Option<Range>) {
        if self.is_intersection_empty(other) {
            return (Some(self.clone()), None);
        } else if self.lower() < other.lower() && self.upper() > other.upper() {
            return (Some(Range::new(self.lower(), other.lower())), Some(Range::new(other.upper(), self.upper())));
        } else if self.lower() < other.lower() {
            return (Some(Range::new(self.lower(), other.lower())), None);
        } else if self.upper() > other.upper() {
            return (None, Some(Range::new(other.upper(), self.upper())));
        }
        (None, None)
    }
}

#[derive(Debug, Default, Clone)]
struct IntervalSet<I> {
    ranges: Vec<Range>,
}

impl<I> IntervalSet<I> {
    pub fn new(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }

    pub fn difference(&mut self, other: &IntervalSet<I>) {
        if self.ranges.is_empty() || other.ranges.is_empty() {
            return;
        }

        // existing difference implementation...
    }
}

#[test]
fn test_difference_no_overlap() {
    let mut set_a = IntervalSet::new(vec![Range::new(1, 5)]);
    let set_b = IntervalSet::new(vec![Range::new(6, 10)]);

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0], Range::new(1, 5));
}

#[test]
fn test_difference_full_overlap() {
    let mut set_a = IntervalSet::new(vec![Range::new(1, 5)]);
    let set_b = IntervalSet::new(vec![Range::new(1, 5)]);

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 0);
}

#[test]
fn test_difference_partial_left_overlap() {
    let mut set_a = IntervalSet::new(vec![Range::new(3, 7)]);
    let set_b = IntervalSet::new(vec![Range::new(1, 5)]);

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0], Range::new(5, 7));
}

#[test]
fn test_difference_partial_right_overlap() {
    let mut set_a = IntervalSet::new(vec![Range::new(1, 7)]);
    let set_b = IntervalSet::new(vec![Range::new(5, 9)]);

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 1);
    assert_eq!(set_a.ranges[0], Range::new(1, 5));
}

#[test]
fn test_difference_multiple_ranges() {
    let mut set_a = IntervalSet::new(vec![Range::new(1, 10)]);
    let set_b = IntervalSet::new(vec![Range::new(3, 5), Range::new(6, 8)]);

    set_a.difference(&set_b);
    assert_eq!(set_a.ranges.len(), 2);
    assert_eq!(set_a.ranges[0], Range::new(1, 3));
    assert_eq!(set_a.ranges[1], Range::new(8, 10));
}

