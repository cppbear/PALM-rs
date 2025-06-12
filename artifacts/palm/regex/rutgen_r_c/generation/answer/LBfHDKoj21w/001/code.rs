// Answer 0

#[test]
fn test_with_end_updates_end_position() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start_position, end_position);
    let new_end_position = Position { offset: 10, line: 1, column: 11 };
    
    let result = span.with_end(new_end_position);
    
    assert_eq!(result.start, start_position);
    assert_eq!(result.end, new_end_position);
}

#[test]
fn test_with_end_empty_span() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::splat(position);
    let new_end_position = Position { offset: 3, line: 1, column: 4 };
    
    let result = span.with_end(new_end_position);
    
    assert_eq!(result.start, position);
    assert_eq!(result.end, new_end_position);
}

#[test]
fn test_with_end_same_position() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start_position, end_position);
    
    let result = span.with_end(end_position);
    
    assert_eq!(result.start, start_position);
    assert_eq!(result.end, end_position);
}

