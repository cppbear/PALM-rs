// Answer 0

#[derive(Debug)]
struct Interval {
    start: usize,
    end: usize,
}

struct IntervalSet {
    ranges: Vec<Interval>,
}

impl IntervalSet {
    fn new() -> Self {
        IntervalSet {
            ranges: Vec::new(),
        }
    }

    pub fn push(&mut self, interval: Interval) {
        self.ranges.push(interval);
        self.canonicalize();
    }

    fn canonicalize(&mut self) {
        // Example logic for canonicalization (not detailed in the original request)
        self.ranges.sort_by(|a, b| a.start.cmp(&b.start));
    }
}

#[test]
fn test_push_single_interval() {
    let mut interval_set = IntervalSet::new();
    let interval = Interval { start: 1, end: 5 };
    interval_set.push(interval);
    assert_eq!(interval_set.ranges.len(), 1);
    assert_eq!(interval_set.ranges[0].start, 1);
    assert_eq!(interval_set.ranges[0].end, 5);
}

#[test]
fn test_push_multiple_intervals() {
    let mut interval_set = IntervalSet::new();
    let interval1 = Interval { start: 1, end: 3 };
    let interval2 = Interval { start: 4, end: 6 };
    interval_set.push(interval1);
    interval_set.push(interval2);
    assert_eq!(interval_set.ranges.len(), 2);
    assert_eq!(interval_set.ranges[0].start, 1);
    assert_eq!(interval_set.ranges[1].start, 4);
}

#[test]
fn test_push_overlapping_intervals() {
    let mut interval_set = IntervalSet::new();
    let interval1 = Interval { start: 1, end: 5 };
    let interval2 = Interval { start: 3, end: 7 };
    interval_set.push(interval1);
    interval_set.push(interval2);
    assert_eq!(interval_set.ranges.len(), 2);
    assert_eq!(interval_set.ranges[0].start, 1);
    assert_eq!(interval_set.ranges[1].start, 3);
}

