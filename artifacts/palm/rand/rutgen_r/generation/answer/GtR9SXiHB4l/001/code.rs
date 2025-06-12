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
fn test_is_empty_with_empty_range() {
    let range = Range { start: 5, end: 5 };
    assert!(range.is_empty());
}

#[test]
fn test_is_empty_with_start_greater_than_end() {
    let range = Range { start: 10, end: 1 };
    assert!(range.is_empty());
}

#[test]
fn test_is_empty_with_negative_range() {
    let range = Range { start: -5, end: -10 };
    assert!(range.is_empty());
}

#[test]
fn test_is_empty_with_start_equal_to_end() {
    let range = Range { start: 0, end: 0 };
    assert!(range.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_range() {
    let range = Range { start: 1, end: 2 };
    assert!(!range.is_empty());
}

