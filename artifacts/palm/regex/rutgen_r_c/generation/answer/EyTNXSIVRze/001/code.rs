// Answer 0

#[test]
fn test_is_empty_with_same_offsets() {
    let position = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(position, position);
    assert!(span.is_empty());
}

#[test]
fn test_is_empty_with_different_offsets() {
    let start_position = Position { offset: 3, line: 1, column: 4 };
    let end_position = Position { offset: 7, line: 1, column: 8 };
    let span = Span::new(start_position, end_position);
    assert!(!span.is_empty());
}

#[test]
fn test_is_empty_with_zero_offset() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(position, position);
    assert!(span.is_empty());
}

#[test]
fn test_is_empty_with_large_offsets() {
    let start_position = Position { offset: 100, line: 10, column: 1 };
    let span = Span::new(start_position, start_position);
    assert!(span.is_empty());
}

#[test]
fn test_is_empty_with_negative_impossible_offsets() {
    let position = Position { offset: usize::MAX, line: 1, column: 1 };
    let span = Span::new(position, position);
    assert!(span.is_empty());
}

