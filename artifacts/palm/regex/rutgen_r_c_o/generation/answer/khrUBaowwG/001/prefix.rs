// Answer 0

#[test]
fn test_with_start_basic() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 10, line: 1, column: 11 };
    let span = Span::new(start_pos, end_pos);
    let new_pos = Position { offset: 5, line: 1, column: 6 };
    span.with_start(new_pos);
}

#[test]
fn test_with_start_edge_case_zero_offset() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start_pos, end_pos);
    let new_pos = Position { offset: 0, line: 1, column: 1 };
    span.with_start(new_pos);
}

#[test]
fn test_with_start_edge_case_max_offset() {
    let start_pos = Position { offset: 1000, line: 100, column: 1000 };
    let end_pos = Position { offset: 1000, line: 100, column: 1000 };
    let span = Span::new(start_pos, end_pos);
    let new_pos = Position { offset: 1000, line: 100, column: 1000 };
    span.with_start(new_pos);
}

#[test]
fn test_with_start_different_line() {
    let start_pos = Position { offset: 1, line: 2, column: 3 };
    let end_pos = Position { offset: 4, line: 1, column: 5 };
    let span = Span::new(start_pos, end_pos);
    let new_pos = Position { offset: 3, line: 3, column: 4 };
    span.with_start(new_pos);
}

#[test]
fn test_with_start_with_high_column() {
    let start_pos = Position { offset: 2, line: 5, column: 1000 };
    let end_pos = Position { offset: 20, line: 5, column: 1000 };
    let span = Span::new(start_pos, end_pos);
    let new_pos = Position { offset: 5, line: 5, column: 999 };
    span.with_start(new_pos);
}

#[test]
fn test_with_start_empty_span() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start_pos, end_pos);
    let new_pos = Position { offset: 1, line: 1, column: 1 };
    span.with_start(new_pos);
}

