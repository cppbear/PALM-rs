// Answer 0

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
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

struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    fn new() -> Self {
        RangeSet { ranges: Vec::new() }
    }

    fn is_canonical(&self) -> bool {
        let mut last_end = 0;
        for range in &self.ranges {
            if range.start < last_end {
                return false;
            }
            last_end = range.end;
        }
        true
    }

    fn canonicalize(&mut self) {
        if self.is_canonical() {
            return;
        }
        self.ranges.sort_by_key(|r| r.start);
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
fn test_canonicalize_already_canonical() {
    let mut set = RangeSet::new();
    set.ranges.push(Range::new(1, 2));
    set.ranges.push(Range::new(3, 4));
    set.ranges.push(Range::new(5, 6));
    
    set.canonicalize();
    
    assert!(set.is_canonical());
}

#[test]
fn test_canonicalize_simple_merge() {
    let mut set = RangeSet::new();
    set.ranges.push(Range::new(1, 3));
    set.ranges.push(Range::new(2, 5));
    
    set.canonicalize();

    assert_eq!(set.ranges.len(), 1);
    assert_eq!(set.ranges[0], Range::new(1, 5));
    assert!(set.is_canonical());
}

#[test]
fn test_canonicalize_multiple_merges() {
    let mut set = RangeSet::new();
    set.ranges.push(Range::new(1, 4));
    set.ranges.push(Range::new(3, 5));
    set.ranges.push(Range::new(6, 8));
    
    set.canonicalize();

    assert_eq!(set.ranges.len(), 2);
    assert_eq!(set.ranges[0], Range::new(1, 5));
    assert_eq!(set.ranges[1], Range::new(6, 8));
    assert!(set.is_canonical());
}

#[test]
fn test_canonicalize_no_ranges() {
    let mut set = RangeSet::new();
    
    set.canonicalize();

    assert!(set.is_canonical());
}

#[test]
fn test_canonicalize_single_range() {
    let mut set = RangeSet::new();
    set.ranges.push(Range::new(1, 2));
    
    set.canonicalize();

    assert_eq!(set.ranges.len(), 1);
    assert_eq!(set.ranges[0], Range::new(1, 2));
    assert!(set.is_canonical());
}

