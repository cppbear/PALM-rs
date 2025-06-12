// Answer 0

#[test]
fn test_span_new_valid_positions() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start, end);
    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
}

#[test]
fn test_span_new_same_positions() {
    let pos = Position { offset: 10, line: 2, column: 5 };
    let span = Span::new(pos, pos);
    assert_eq!(span.start, pos);
    assert_eq!(span.end, pos);
}

#[test]
fn test_span_new_reverse_positions() {
    let start = Position { offset: 10, line: 1, column: 5 };
    let end = Position { offset: 5, line: 1, column: 1 };
    let span = Span::new(start, end);
    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
}

