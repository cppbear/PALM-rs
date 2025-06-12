// Answer 0

#[test]
fn test_with_end_updates_end_position() {
    let start_position = Position { offset: 0, line: 1, column: 1 };
    let end_position = Position { offset: 5, line: 1, column: 6 };
    let new_end_position = Position { offset: 10, line: 1, column: 11 };

    let span = Span::new(start_position, end_position);
    let updated_span = span.with_end(new_end_position);

    assert_eq!(updated_span.end, new_end_position);
    assert_eq!(updated_span.start, start_position);
}

#[test]
fn test_with_end_retains_start_position() {
    let start_position = Position { offset: 1, line: 2, column: 2 };
    let end_position = Position { offset: 3, line: 2, column: 4 };
    let new_end_position = Position { offset: 8, line: 3, column: 1 };

    let span = Span::new(start_position, end_position);
    let updated_span = span.with_end(new_end_position);

    assert_eq!(updated_span.start, start_position);
    assert_ne!(updated_span.end, end_position); // ensure end position is updated
}

#[test]
fn test_with_end_does_not_change_mid_span() {
    let start_position = Position { offset: 5, line: 1, column: 6 };
    let end_position = Position { offset: 15, line: 1, column: 16 };
    let new_end_position = Position { offset: 20, line: 1, column: 21 };

    let span = Span::new(start_position, end_position);
    let updated_span = span.with_end(new_end_position);

    assert_eq!(updated_span.end, new_end_position);
    assert!(updated_span.start != end_position); // ensures start position still remains the same
}

