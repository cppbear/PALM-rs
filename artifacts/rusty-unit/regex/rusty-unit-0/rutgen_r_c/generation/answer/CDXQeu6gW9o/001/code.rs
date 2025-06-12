// Answer 0

#[test]
fn test_new_span_valid_positions() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 10, line: 1, column: 11 };
    let span = Span::new(start, end);
    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
}

#[test]
fn test_new_span_same_start_end() {
    let pos = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(pos, pos);
    assert_eq!(span.start, pos);
    assert_eq!(span.end, pos);
}

#[test]
fn test_new_span_multiline() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 20, line: 2, column: 5 };
    let span = Span::new(start, end);
    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
}

#[test]
fn test_new_span_large_offsets() {
    let start = Position { offset: usize::MAX - 1, line: 10, column: 10 };
    let end = Position { offset: usize::MAX, line: 10, column: 11 };
    let span = Span::new(start, end);
    assert_eq!(span.start, start);
    assert_eq!(span.end, end);
}

