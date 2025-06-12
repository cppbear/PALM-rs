// Answer 0

#[derive(Debug)]
struct Range {
    start: char,
    end: char,
}

impl Range {
    pub fn new(start: char, end: char) -> Self {
        Range { start, end }
    }

    pub fn end(&self) -> char {
        self.end
    }
}

#[test]
fn test_end_return_value() {
    let range = Range::new('a', 'z');
    assert_eq!(range.end(), 'z');
}

#[test]
fn test_end_boundary_condition() {
    let range = Range::new('x', 'x');
    assert_eq!(range.end(), 'x');
}

#[test]
fn test_end_greater_than_start() {
    let range = Range::new('a', 'b');
    assert!(range.end() >= range.start);
}

