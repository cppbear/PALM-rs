// Answer 0

#[test]
fn test_formatter_with_multiple_lines_and_spans() {
    let pattern = "abc\ndef\nghi"; // Length is within 1 to 1000 characters and contains newlines
    let line_number_width = 5; // Valid line number width
    let stderr = "Some error occurred"; // Error message
    let span1 = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 3 } };
    let span2 = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 3 } };
    let span3 = ast::Span { start: Position { line: 2, column: 0 }, end: Position { line: 2, column: 3 } };
    let multi_span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 2, column: 3 } };

    let formatter = Formatter {
        pattern: pattern,
        err: &stderr,
        span: &span1,
        aux_span: None,
    };

    let mut spans = Spans {
        pattern: pattern,
        line_number_width: line_number_width,
        by_line: vec![vec![span1], vec![span2], vec![span3]],
        multi_line: vec![multi_span],
    };

    let _ = writeln!(std::io::stdout(), "{}", formatter);
}

#[test]
fn test_formatter_with_overlapping_spans() {
    let pattern = "hello\nworld\nrust\nlanguage"; // Contains newlines
    let line_number_width = 3; // Valid line number width
    let stderr = "Another error"; // Error message
    let span1 = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: 5 } };
    let span2 = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 1, column: 5 } };
    let multi_span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 1, column: 5 } };

    let formatter = Formatter {
        pattern: pattern,
        err: &stderr,
        span: &span1,
        aux_span: None,
    };

    let mut spans = Spans {
        pattern: pattern,
        line_number_width: line_number_width,
        by_line: vec![vec![span1], vec![span2]],
        multi_line: vec![multi_span],
    };

    let _ = writeln!(std::io::stdout(), "{}", formatter);
}

#[test]
fn test_formatter_with_error_on_single_line() {
    let pattern = "single line"; // No newline
    let line_number_width = 0; // No line numbers
    let stderr = "Single line error"; // Error message
   
    let formatter = Formatter {
        pattern: pattern,
        err: &stderr,
        span: &ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 0, column: pattern.len() } },
        aux_span: None,
    };

    let mut spans = Spans {
        pattern: pattern,
        line_number_width: line_number_width,
        by_line: vec![vec![]],
        multi_line: vec![],
    };

    let _ = writeln!(std::io::stdout(), "{}", formatter);
}

#[test]
fn test_formatter_with_empty_err_message() {
    let pattern = "line one\nline two\nline three"; // Valid pattern with newlines
    let line_number_width = 2; // Valid line number width
    let stderr = ""; // Empty error message
    let span1 = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 1, column: 5 } };

    let formatter = Formatter {
        pattern: pattern,
        err: &stderr,
        span: &span1,
        aux_span: None,
    };

    let mut spans = Spans {
        pattern: pattern,
        line_number_width: line_number_width,
        by_line: vec![vec![span1]],
        multi_line: vec![span1],
    };

    let _ = writeln!(std::io::stdout(), "{}", formatter);
}

