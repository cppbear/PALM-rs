// Answer 0

#[derive(Debug)]
struct Span {
    start: Position,
    end: Position,
}

#[derive(Debug)]
struct Position {
    offset: usize,
}

impl Span {
    pub fn is_empty(&self) -> bool {
        self.start.offset == self.end.offset
    }
}

#[test]
fn test_is_empty_with_equal_offsets() {
    let span = Span {
        start: Position { offset: 5 },
        end: Position { offset: 5 },
    };
    assert!(span.is_empty());
}

#[test]
fn test_is_empty_with_different_offsets() {
    let span = Span {
        start: Position { offset: 3 },
        end: Position { offset: 7 },
    };
    assert!(!span.is_empty());
}

#[test]
fn test_is_empty_with_zero_offset() {
    let span = Span {
        start: Position { offset: 0 },
        end: Position { offset: 0 },
    };
    assert!(span.is_empty());
}

#[test]
fn test_is_empty_with_large_offsets() {
    let span = Span {
        start: Position { offset: 100_000 },
        end: Position { offset: 100_000 },
    };
    assert!(span.is_empty());
}

#[test]
fn test_is_empty_with_large_different_offsets() {
    let span = Span {
        start: Position { offset: 1_000_000 },
        end: Position { offset: 1_000_001 },
    };
    assert!(!span.is_empty());
}

