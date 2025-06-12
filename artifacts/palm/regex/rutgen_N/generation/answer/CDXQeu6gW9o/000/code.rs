// Answer 0

#[derive(Debug, PartialEq)]
struct Position {
    value: usize,
}

#[derive(Debug, PartialEq)]
struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }
}

#[test]
fn test_new_span_with_valid_positions() {
    let start = Position { value: 1 };
    let end = Position { value: 5 };
    let span = Span::new(start.clone(), end.clone());
    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
}

#[test]
fn test_new_span_with_equal_positions() {
    let pos = Position { value: 3 };
    let span = Span::new(pos.clone(), pos.clone());
    assert_eq!(span.start, pos);
    assert_eq!(span.end, pos);
}

#[test]
fn test_new_span_with_boundary_positions() {
    let start = Position { value: 0 };
    let end = Position { value: 0 };
    let span = Span::new(start.clone(), end.clone());
    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
}

