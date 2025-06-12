// Answer 0

#[test]
fn test_with_start_updates_start_position() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start_pos, end_pos);
    
    let new_start_pos = Position { offset: 2, line: 1, column: 3 };
    let updated_span = span.with_start(new_start_pos);
    
    assert_eq!(updated_span.start, new_start_pos);
    assert_eq!(updated_span.end, end_pos);
}

#[test]
fn test_with_start_leaves_end_position_unchanged() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start_pos, end_pos);
    
    let new_start_pos = Position { offset: 2, line: 1, column: 3 };
    let updated_span = span.with_start(new_start_pos);
    
    assert_eq!(span.end, updated_span.end);
}

#[test]
fn test_with_start_does_not_change_span_if_same_start_position() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 5, line: 1, column: 6 };
    let span = Span::new(start_pos, end_pos);
    
    let updated_span = span.with_start(start_pos);
    
    assert_eq!(span, updated_span);
}

