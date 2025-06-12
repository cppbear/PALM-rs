// Answer 0

#[test]
fn test_span_valid_inputs() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 5, line: 1, column: 6 };
    let span = Span { start, end };
    let _ = format!("{:?}", span);
}

#[test]
fn test_span_start_equals_end() {
    let start = Position { offset: 3, line: 2, column: 4 };
    let end = Position { offset: 3, line: 2, column: 4 };
    let span = Span { start, end };
    let _ = format!("{:?}", span);
}

#[test]
fn test_span_edge_conditions() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 100, line: 100, column: 100 };
    let span = Span { start, end };
    let _ = format!("{:?}", span);
}

#[test]
fn test_span_column_increment() {
    let start = Position { offset: 10, line: 5, column: 10 };
    let end = Position { offset: 15, line: 5, column: 15 };
    let span = Span { start, end };
    let _ = format!("{:?}", span);
}

#[test]
fn test_span_multiple_lines() {
    let start = Position { offset: 18, line: 2, column: 9 };
    let end = Position { offset: 25, line: 3, column: 5 };
    let span = Span { start, end };
    let _ = format!("{:?}", span);
}

