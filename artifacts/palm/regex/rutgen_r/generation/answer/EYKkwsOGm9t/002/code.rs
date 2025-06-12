// Answer 0

#[derive(Debug)]
struct Range {
    start: u8,
    end: u8,
}

impl Range {
    pub fn matches(&self, byte: u8) -> bool {
        self.start <= byte && byte <= self.end
    }
}

#[test]
fn test_matches_lower_bound() {
    let range = Range { start: 10, end: 20 };
    assert_eq!(range.matches(9), false);
}

#[test]
fn test_matches_exact_lower_bound() {
    let range = Range { start: 10, end: 20 };
    assert_eq!(range.matches(10), true);
}

#[test]
fn test_matches_internal_value() {
    let range = Range { start: 10, end: 20 };
    assert_eq!(range.matches(15), true);
}

#[test]
fn test_matches_exact_upper_bound() {
    let range = Range { start: 10, end: 20 };
    assert_eq!(range.matches(20), true);
}

#[test]
fn test_matches_upper_bound() {
    let range = Range { start: 10, end: 20 };
    assert_eq!(range.matches(21), false);
}

#[test]
fn test_matches_empty_range() {
    let range = Range { start: 10, end: 9 };
    assert_eq!(range.matches(10), false);
}

