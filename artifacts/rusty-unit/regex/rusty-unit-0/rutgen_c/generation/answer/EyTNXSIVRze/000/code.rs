// Answer 0

#[test]
fn test_span_is_empty_true() {
    let start_pos = Position { offset: 5, line: 1, column: 6 };
    let end_pos = Position { offset: 5, line: 1, column: 6 };
    let span = Span { start: start_pos, end: end_pos };
    assert!(span.is_empty());
}

#[test]
fn test_span_is_empty_false() {
    let start_pos = Position { offset: 5, line: 1, column: 6 };
    let end_pos = Position { offset: 10, line: 1, column: 11 };
    let span = Span { start: start_pos, end: end_pos };
    assert!(!span.is_empty());
}

#[test]
fn test_span_is_empty_edge_case() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 0, line: 1, column: 1 };
    let span = Span { start: start_pos, end: end_pos };
    assert!(span.is_empty());
}

