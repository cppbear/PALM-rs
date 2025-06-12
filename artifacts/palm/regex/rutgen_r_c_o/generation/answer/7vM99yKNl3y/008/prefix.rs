// Answer 0

#[test]
fn test_from_formatter_case_1() {
    let pattern = "a\nb";
    let start_position = Position { line: 1, column: 0 };
    let end_position = Position { line: 1, column: 1 };
    let span = ast::Span { start: start_position, end: end_position };
    let aux_span = Some(span.clone());

    let formatter = Formatter {
        pattern: pattern,
        err: &"error",
        span: &span,
        aux_span: aux_span,
    };

    let spans = Spans::from_formatter(&formatter);
}

#[test]
fn test_from_formatter_case_2() {
    let pattern = "line 1\nline 2";
    let start_position = Position { line: 1, column: 0 };
    let end_position = Position { line: 2, column: 5 };
    let span = ast::Span { start: start_position, end: end_position };
    let aux_span = Some(span.clone());

    let formatter = Formatter {
        pattern: pattern,
        err: &"error",
        span: &span,
        aux_span: aux_span,
    };

    let spans = Spans::from_formatter(&formatter);
}

#[test]
fn test_from_formatter_case_3() {
    let pattern = "first\nsecond\nthird";
    let start_position = Position { line: 2, column: 0 };
    let end_position = Position { line: 2, column: 6 };
    let span = ast::Span { start: start_position, end: end_position };
    let aux_span = Some(span.clone());

    let formatter = Formatter {
        pattern: pattern,
        err: &"error",
        span: &span,
        aux_span: aux_span,
    };

    let spans = Spans::from_formatter(&formatter);
}

#[test]
fn test_from_formatter_case_4() {
    let pattern = "line one\nline two\nline three\n";
    let start_position = Position { line: 3, column: 0 };
    let end_position = Position { line: 3, column: 11 };
    let span = ast::Span { start: start_position, end: end_position };
    let aux_span = Some(span.clone());

    let formatter = Formatter {
        pattern: pattern,
        err: &"error",
        span: &span,
        aux_span: aux_span,
    };

    let spans = Spans::from_formatter(&formatter);
}

#[test]
fn test_from_formatter_case_5() {
    let pattern = "abc\nxyz\n123\n";
    let start_position = Position { line: 1, column: 0 };
    let end_position = Position { line: 1, column: 3 };
    let span = ast::Span { start: start_position, end: end_position };
    let aux_span = Some(span.clone());

    let formatter = Formatter {
        pattern: pattern,
        err: &"error",
        span: &span,
        aux_span: aux_span,
    };

    let spans = Spans::from_formatter(&formatter);
}

