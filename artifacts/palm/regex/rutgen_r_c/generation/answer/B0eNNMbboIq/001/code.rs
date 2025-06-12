// Answer 0

#[test]
fn test_is_one_line_same_line() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 10, line: 1, column: 11 };
    let span = Span::new(start, end);
    assert!(span.is_one_line());
}

#[test]
fn test_is_one_line_different_line() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 10, line: 2, column: 1 };
    let span = Span::new(start, end);
    assert!(!span.is_one_line());
}

#[test]
fn test_is_one_line_same_line_boundary() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start, end);
    assert!(span.is_one_line());
}

#[test]
fn test_is_one_line_empty_span() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start, end);
    assert!(span.is_empty());
    assert!(span.is_one_line());
}

