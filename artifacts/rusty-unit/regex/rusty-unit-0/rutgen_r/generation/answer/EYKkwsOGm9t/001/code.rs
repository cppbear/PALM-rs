// Answer 0

#[derive(Debug)]
struct ByteRange {
    start: u8,
    end: u8,
}

impl ByteRange {
    pub fn matches(&self, byte: u8) -> bool {
        self.start <= byte && byte <= self.end
    }
}

#[test]
fn test_matches_start_boundary() {
    let range = ByteRange { start: 5, end: 10 };
    assert_eq!(range.matches(5), true);
}

#[test]
fn test_matches_end_boundary() {
    let range = ByteRange { start: 5, end: 10 };
    assert_eq!(range.matches(10), true);
}

#[test]
fn test_matches_inside_range() {
    let range = ByteRange { start: 5, end: 10 };
    assert_eq!(range.matches(7), true);
}

#[test]
fn test_matches_below_range() {
    let range = ByteRange { start: 5, end: 10 };
    assert_eq!(range.matches(4), false);
}

#[test]
fn test_matches_above_range() {
    let range = ByteRange { start: 5, end: 10 };
    assert_eq!(range.matches(11), false);
}

