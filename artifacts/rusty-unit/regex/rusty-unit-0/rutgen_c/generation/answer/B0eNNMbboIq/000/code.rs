// Answer 0

#[test]
fn test_is_one_line_same_line() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 10, line: 1, column: 11 };
    let span = Span::new(start, end);
    
    assert!(span.is_one_line());
}

#[test]
fn test_is_one_line_different_lines() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 10, line: 2, column: 1 };
    let span = Span::new(start, end);

    assert!(!span.is_one_line());
}

#[test]
fn test_is_one_line_empty() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::splat(position);
    
    assert!(span.is_one_line());
}

