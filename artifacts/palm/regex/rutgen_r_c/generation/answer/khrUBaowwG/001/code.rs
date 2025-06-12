// Answer 0

#[test]
fn test_with_start() {
    let start_pos = Position { offset: 5, line: 1, column: 6 };
    let end_pos = Position { offset: 15, line: 1, column: 16 };
    let span = Span::new(start_pos, end_pos);

    let new_start_pos = Position { offset: 2, line: 1, column: 3 };
    let new_span = span.with_start(new_start_pos);

    assert_eq!(new_span.start, new_start_pos);
    assert_eq!(new_span.end, end_pos);
}

#[test]
fn test_with_start_same_position() {
    let start_pos = Position { offset: 10, line: 2, column: 1 };
    let end_pos = Position { offset: 20, line: 2, column: 11 };
    let span = Span::new(start_pos, end_pos);

    let new_span = span.with_start(start_pos);
    
    assert_eq!(new_span.start, start_pos);
    assert_eq!(new_span.end, end_pos);
}

#[test]
fn test_with_start_different_lines() {
    let start_pos = Position { offset: 30, line: 3, column: 5 };
    let end_pos = Position { offset: 40, line: 4, column: 10 };
    let span = Span::new(start_pos, end_pos);

    let new_start_pos = Position { offset: 25, line: 2, column: 5 };
    let new_span = span.with_start(new_start_pos);

    assert_eq!(new_span.start, new_start_pos);
    assert_eq!(new_span.end, end_pos);
}

