// Answer 0

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }
}

#[test]
fn test_is_empty_when_range_is_empty() {
    let range = Range { start: 5, end: 5 };
    assert!(range.is_empty());
}

#[test]
fn test_is_empty_when_range_is_not_empty() {
    let range = Range { start: 2, end: 5 };
    assert!(!range.is_empty());
}

#[test]
fn test_is_empty_when_range_is_negative() {
    let range = Range { start: -5, end: -3 };
    assert!(!range.is_empty());
}

#[test]
fn test_is_empty_when_range_starts_greater_than_end() {
    let range = Range { start: 7, end: 3 };
    assert!(range.is_empty());
}

