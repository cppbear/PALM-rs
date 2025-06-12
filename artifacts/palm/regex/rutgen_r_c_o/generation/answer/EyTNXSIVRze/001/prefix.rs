// Answer 0

#[test]
fn test_span_is_empty_case_0() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start, end);
    span.is_empty();
}

#[test]
fn test_span_is_empty_case_1() {
    let start = Position { offset: 1, line: 1, column: 2 };
    let end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(start, end);
    span.is_empty();
}

#[test]
fn test_span_is_empty_case_2() {
    let start = Position { offset: 2, line: 1, column: 3 };
    let end = Position { offset: 2, line: 1, column: 3 };
    let span = Span::new(start, end);
    span.is_empty();
}

#[test]
fn test_span_is_empty_case_max_int() {
    let start = Position { offset: 2147483647, line: 1, column: 2147483648 };
    let end = Position { offset: 2147483647, line: 1, column: 2147483648 };
    let span = Span::new(start, end);
    span.is_empty();
}

#[test]
fn test_span_is_empty_case_large_offset() {
    let start = Position { offset: 4294967295, line: 1, column: 4294967296 };
    let end = Position { offset: 4294967295, line: 1, column: 4294967296 };
    let span = Span::new(start, end);
    span.is_empty();
}

#[test]
fn test_span_is_empty_case_100() {
    let start = Position { offset: 100, line: 1, column: 101 };
    let end = Position { offset: 100, line: 1, column: 101 };
    let span = Span::new(start, end);
    span.is_empty();
}

#[test]
fn test_span_is_empty_case_99() {
    let start = Position { offset: 99, line: 1, column: 100 };
    let end = Position { offset: 99, line: 1, column: 100 };
    let span = Span::new(start, end);
    span.is_empty();
}

