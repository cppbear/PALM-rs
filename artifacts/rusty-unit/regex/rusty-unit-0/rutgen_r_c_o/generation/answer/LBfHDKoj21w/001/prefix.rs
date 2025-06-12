// Answer 0

#[test]
fn test_with_end_normal_case() {
    let start = Position { offset: 10, line: 1, column: 1 };
    let end = Position { offset: 20, line: 1, column: 2 };
    let span = Span::new(start, end);
    let new_end = Position { offset: 30, line: 1, column: 3 };
    span.with_end(new_end);
}

#[test]
fn test_with_end_edge_case_start_equals_end() {
    let start = Position { offset: 50, line: 5, column: 25 };
    let end = Position { offset: 50, line: 5, column: 25 };
    let span = Span::new(start, end);
    let new_end = Position { offset: 50, line: 5, column: 26 };
    span.with_end(new_end);
}

#[test]
fn test_with_end_edge_case_maximum_offsets() {
    let start = Position { offset: 999, line: 100, column: 100 };
    let end = Position { offset: 999, line: 100, column: 100 };
    let span = Span::new(start, end);
    let new_end = Position { offset: 1000, line: 100, column: 100 };
    span.with_end(new_end);
}

#[test]
fn test_with_end_edge_case_minimum_offsets() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start, end);
    let new_end = Position { offset: 1, line: 1, column: 1 };
    span.with_end(new_end);
}

#[test]
fn test_with_end_empty_span() {
    let start = Position { offset: 5, line: 2, column: 5 };
    let end = Position { offset: 5, line: 2, column: 5 };
    let span = Span::new(start, end);
    let new_end = Position { offset: 10, line: 2, column: 10 };
    span.with_end(new_end);
}

