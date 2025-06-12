// Answer 0

#[derive(Debug, PartialEq, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn union(&self, other: &Range) -> Option<Range> {
        if self.end < other.start || other.end < self.start {
            None
        } else {
            Some(Range {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        }
    }
}

struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    fn is_canonical(&self) -> bool {
        // Assume this is not canonical for testing
        false
    }

    fn canonicalize(&mut self) {
        if self.is_canonical() {
            return;
        }
        self.ranges.sort();
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
fn test_canonicalize_non_canonical_with_non_empty_ranges() {
    let mut range_set = RangeSet {
        ranges: vec![
            Range { start: 1, end: 3 },
            Range { start: 2, end: 5 },
            Range { start: 6, end: 8 },
        ],
    };
    
    range_set.canonicalize();
    
    assert_eq!(range_set.ranges.len(), 2);
    assert_eq!(range_set.ranges[0], Range { start: 1, end: 5 });
    assert_eq!(range_set.ranges[1], Range { start: 6, end: 8 });
}

#[test]
#[should_panic]
fn test_canonicalize_should_panic_on_drain_when_unexpected() {
    let mut range_set = RangeSet {
        ranges: vec![
            Range { start: 0, end: 1 },
            Range { start: 3, end: 5 },
        ],
    };

    // Simulating a scenario that could cause panic
    range_set.ranges.push(Range { start: 1, end: 2 }); // Introduce element to potentially force a panic
    range_set.canonicalize();
}

