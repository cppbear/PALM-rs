// Answer 0

#[test]
fn test_is_one_line_true_same_line() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 10, line: 1, column: 11 };
    let span = Span::new(start, end);
    span.is_one_line();
}

#[test]
fn test_is_one_line_true_same_line_high_value() {
    let start = Position { offset: 50, line: 1000, column: 1 };
    let end = Position { offset: 60, line: 1000, column: 11 };
    let span = Span::new(start, end);
    span.is_one_line();
}

#[test]
fn test_is_one_line_false_different_lines() {
    let start = Position { offset: 5, line: 1, column: 6 };
    let end = Position { offset: 15, line: 2, column: 5 };
    let span = Span::new(start, end);
    span.is_one_line();
}

#[test]
fn test_is_one_line_false_start_line_less_end_line() {
    let start = Position { offset: 20, line: 2, column: 6 };
    let end = Position { offset: 25, line: 3, column: 5 };
    let span = Span::new(start, end);
    span.is_one_line();
}

#[test]
fn test_is_one_line_empty() {
    let position = Position { offset: 0, line: 1, column: 1 };
    let span = Span::splat(position);
    span.is_one_line();
}

#[test]
fn test_is_one_line_start_equals_end() {
    let position = Position { offset: 0, line: 5, column: 5 };
    let span = Span::new(position, position);
    span.is_one_line();
}

