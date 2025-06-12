// Answer 0

#[test]
fn test_splat_creates_span_with_same_start_and_end() {
    let position = Position { offset: 5, line: 1, column: 6 };
    let span = Span::splat(position);
    assert_eq!(span.start, position);
    assert_eq!(span.end, position);
}

#[test]
fn test_splat_creates_span_with_zero_position() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::splat(position);
    assert_eq!(span.start, position);
    assert_eq!(span.end, position);
}

#[test]
fn test_splat_creates_span_with_large_values() {
    let position = Position { offset: 1000, line: 50, column: 20 };
    let span = Span::splat(position);
    assert_eq!(span.start, position);
    assert_eq!(span.end, position);
}

