// Answer 0

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn start(&self) -> i32 {
        self.start
    }

    fn end(&self) -> i32 {
        self.end
    }

    fn is_empty(&self) -> bool {
        !(self.start() <= self.end())
    }
}

#[test]
fn test_is_empty_range_empty() {
    let range = Range { start: 5, end: 3 };
    assert_eq!(range.is_empty(), true);
}

#[test]
fn test_is_empty_range_non_empty() {
    let range = Range { start: 1, end: 4 };
    assert_eq!(range.is_empty(), false);
}

#[test]
fn test_is_empty_range_at_boundaries() {
    let range = Range { start: 0, end: 0 };
    assert_eq!(range.is_empty(), false);
} 

#[test]
fn test_is_empty_range_negative_values() {
    let range = Range { start: -2, end: -5 };
    assert_eq!(range.is_empty(), true);
} 

#[test]
fn test_is_empty_range_equal_start_end() {
    let range = Range { start: 10, end: 10 };
    assert_eq!(range.is_empty(), false);
}

