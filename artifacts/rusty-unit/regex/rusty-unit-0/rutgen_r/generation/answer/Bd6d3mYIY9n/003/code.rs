// Answer 0

fn get_ranges() -> Vec<(u32, u32)> {
    vec![
        (1, 3),
        (4, 5),
        (6, 8),
        (9, 10)
    ]
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Range { start, end }
    }

    fn is_contiguous(&self, other: &Range) -> bool {
        self.end + 1 == other.start || other.end + 1 == self.start
    }
}

struct Class {
    ranges: Vec<Range>,
}

impl Class {
    fn new(ranges: Vec<Range>) -> Self {
        Class { ranges }
    }

    fn is_canonical(&self) -> bool {
        for pair in self.ranges.windows(2) {
            if pair[0].start >= pair[1].start {
                return false;
            }
            if pair[0].is_contiguous(&pair[1]) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_is_canonical_true() {
    let ranges = vec![
        Range::new(1, 3),
        Range::new(4, 5),
        Range::new(6, 8),
        Range::new(9, 10),
    ];
    let class = Class::new(ranges);
    assert!(class.is_canonical());
}

#[test]
fn test_is_canonical_false_due_to_non_canonical_order() {
    let ranges = vec![
        Range::new(5, 7),
        Range::new(4, 6), // this violates the canonical ordering
    ];
    let class = Class::new(ranges);
    assert!(!class.is_canonical());
}

#[test]
fn test_is_canonical_false_due_to_contiguity() {
    let ranges = vec![
        Range::new(1, 2),
        Range::new(2, 3), // this violates the non-contiguous requirement
    ];
    let class = Class::new(ranges);
    assert!(!class.is_canonical());
}

#[test]
fn test_is_canonical_single_range() {
    let ranges = vec![
        Range::new(1, 2),
    ];
    let class = Class::new(ranges);
    assert!(class.is_canonical());
}

#[test]
fn test_is_canonical_empty() {
    let ranges: Vec<Range> = Vec::new();
    let class = Class::new(ranges);
    assert!(class.is_canonical());
}

