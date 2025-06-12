// Answer 0

#[derive(Debug, Clone, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    fn union(&self, other: &Range) -> Option<Range> {
        if self.end >= other.start {
            Some(Range::new(self.start.min(other.start), self.end.max(other.end)))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    fn new() -> Self {
        RangeSet { ranges: Vec::new() }
    }
    
    fn is_canonical(&self) -> bool {
        // Placeholder for actual implementation
        true
    }

    fn canonicalize(&mut self) {
        if self.is_canonical() {
            return;
        }
        self.ranges.sort_by(|a, b| a.start.cmp(&b.start()));
        assert!(!self.ranges.is_empty());

        let drain_end = self.ranges.len();
        for oldi in 0..drain_end {
            if self.ranges.len() > drain_end {
                let (last, rest) = self.ranges.split_last_mut().unwrap();
                if let Some(union) = last.union(&rest[oldi]) {
                    *last = union;
                    continue;
                }
            }
            let range = self.ranges[oldi].clone();
            self.ranges.push(range);
        }
        self.ranges.drain(..drain_end);
    }
}

#[test]
fn test_canonicalize_single_range() {
    let mut set = RangeSet::new();
    set.ranges.push(Range::new(1, 2));
    set.canonicalize();
    assert_eq!(set.ranges, vec![Range::new(1, 2)]);
}

#[test]
fn test_canonicalize_overlapping_ranges() {
    let mut set = RangeSet::new();
    set.ranges.push(Range::new(1, 5));
    set.ranges.push(Range::new(4, 7));
    set.canonicalize();
    assert_eq!(set.ranges, vec![Range::new(1, 7)]);
}

#[test]
fn test_canonicalize_non_overlapping_ranges() {
    let mut set = RangeSet::new();
    set.ranges.push(Range::new(1, 3));
    set.ranges.push(Range::new(4, 6));
    set.canonicalize();
    assert_eq!(set.ranges, vec![Range::new(1, 3), Range::new(4, 6)]);
}

#[test]
#[should_panic]
fn test_canonicalize_empty_range_set() {
    let mut set = RangeSet::new();
    set.canonicalize(); // should panic due to empty ranges
}

