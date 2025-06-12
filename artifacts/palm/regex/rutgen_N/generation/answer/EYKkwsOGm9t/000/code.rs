// Answer 0

#[derive(Debug)]
struct Range {
    start: u8,
    end: u8,
}

impl Range {
    pub fn new(start: u8, end: u8) -> Self {
        Range { start, end }
    }

    pub fn matches(&self, byte: u8) -> bool {
        self.start <= byte && byte <= self.end
    }
}

#[test]
fn test_matches_within_range() {
    let range = Range::new(5, 10);
    assert!(range.matches(7));
}

#[test]
fn test_matches_at_start_bound() {
    let range = Range::new(5, 10);
    assert!(range.matches(5));
}

#[test]
fn test_matches_at_end_bound() {
    let range = Range::new(5, 10);
    assert!(range.matches(10));
}

#[test]
fn test_does_not_match_below_range() {
    let range = Range::new(5, 10);
    assert!(!range.matches(4));
}

#[test]
fn test_does_not_match_above_range() {
    let range = Range::new(5, 10);
    assert!(!range.matches(11));
}

