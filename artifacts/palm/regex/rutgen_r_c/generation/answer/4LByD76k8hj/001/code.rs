// Answer 0

#[test]
fn test_splat_with_valid_position() {
    let pos = Position { offset: 10, line: 1, column: 5 };
    let span = Span::splat(pos);
    assert_eq!(span.start, pos);
    assert_eq!(span.end, pos);
}

#[test]
fn test_splat_with_zero_position() {
    let pos = Position { offset: 0, line: 1, column: 1 };
    let span = Span::splat(pos);
    assert_eq!(span.start, pos);
    assert_eq!(span.end, pos);
}

#[test]
fn test_splat_with_large_offset() {
    let pos = Position { offset: 1000, line: 10, column: 20 };
    let span = Span::splat(pos);
    assert_eq!(span.start, pos);
    assert_eq!(span.end, pos);
}

