// Answer 0

#[test]
fn test_formatter_multiline_error() {
    let pattern = "a\nb\nc";
    let error_message = "syntax error";
    let span_start = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 2 } };
    let span_end = ast::Span { start: Position { line: 2, column: 1 }, end: Position { line: 2, column: 2 } };
    let multi_line_spans = vec![span_start, span_end];
    let spans = Spans {
        pattern,
        line_number_width: 2,
        by_line: vec![vec![span_start], vec![span_end], vec![]],
        multi_line: multi_line_spans,
    };
    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span_start,
        aux_span: None,
    };
    let mut output = String::new();
    formatter.fmt(&mut output).unwrap();
}

#[test]
fn test_formatter_maximum_length() {
    let pattern = "a".repeat(1000) + "\n" + "b".repeat(1000);
    let error_message = "error message";
    let span_start = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 2 } };
    let span_end = ast::Span { start: Position { line: 2, column: 1 }, end: Position { line: 2, column: 2 } };
    let multi_line_spans = vec![span_start, span_end];
    let spans = Spans {
        pattern: &pattern,
        line_number_width: 2,
        by_line: vec![vec![span_start], vec![span_end]],
        multi_line: multi_line_spans,
    };
    let formatter = Formatter {
        pattern: &pattern,
        err: &error_message,
        span: &span_start,
        aux_span: None,
    };
    let mut output = String::new();
    formatter.fmt(&mut output).unwrap();
}

#[test]
fn test_formatter_no_auxiliary_span() {
    let pattern = "line 1\nline 2";
    let error_message = "capture group name duplicate";
    let span_start = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 5 } };
    let span_end = ast::Span { start: Position { line: 2, column: 1 }, end: Position { line: 2, column: 5 } };
    let multi_line_spans = vec![span_start, span_end];
    let spans = Spans {
        pattern,
        line_number_width: 3,
        by_line: vec![vec![span_start], vec![span_end]],
        multi_line: multi_line_spans,
    };
    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span_start,
        aux_span: None,
    };
    let mut output = String::new();
    formatter.fmt(&mut output).unwrap();
}

#[test]
fn test_formatter_multiple_lines_multiple_spans() {
    let pattern = "abc\ndef\nghi";
    let error_message = "invalid escape";
    let span1 = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 4 } };
    let span2 = ast::Span { start: Position { line: 2, column: 1 }, end: Position { line: 2, column: 4 } };
    let multi_line_spans = vec![span1.clone(), span2.clone()];
    let spans = Spans {
        pattern,
        line_number_width: 2,
        by_line: vec![vec![span1], vec![span2]],
        multi_line: multi_line_spans,
    };
    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span1,
        aux_span: None,
    };
    let mut output = String::new();
    formatter.fmt(&mut output).unwrap();
}

#[test]
fn test_formatter_without_multi_line() {
    let pattern = "single line";
    let error_message = "unexpected character";
    let span = ast::Span { start: Position { line: 1, column: 1 }, end: Position { line: 1, column: 12 } };
    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line: vec![vec![span]],
        multi_line: vec![],
    };
    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span: None,
    };
    let mut output = String::new();
    formatter.fmt(&mut output).unwrap();
}

