// Answer 0

#[test]
fn test_formatter_with_multiline_pattern() {
    let pattern = "abc\ndef\nghi";
    let error_message = "Invalid syntax";
    let span = ast::Span { start: Position { line: 1, column: 0 }, end: Position { line: 3, column: 3 } };
    
    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span: None,
    };
    
    let mut output = String::new();
    let _ = write!(&mut output, "{}", formatter);
}

#[test]
fn test_formatter_with_empty_aux_span() {
    let pattern = "abc\ndef";
    let error_message = "Unexpected token";
    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 1, column: 3 } };
    
    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span: None,
    };

    let mut output = String::new();
    let _ = write!(&mut output, "{}", formatter);
}

#[test]
fn test_formatter_with_single_multiline_span() {
    let pattern = "first line\nsecond line\nthird line";
    let error_message = "Too many captures";
    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 2, column: 11 } };

    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span: None,
    };

    let multi_line_span = ast::Span { start: Position { line: 1, column: 4 }, end: Position { line: 1, column: 10 } };
    let mut spans = Spans {
        pattern,
        line_number_width: 5,
        by_line: vec![vec![span], vec![multi_line_span]],
        multi_line: vec![span],
    };

    spans.add(multi_line_span);

    let mut output = String::new();
    let _ = write!(&mut output, "{}", formatter);
}

#[test]
#[should_panic]
fn test_formatter_with_panic_on_large_input() {
    let pattern = "a\nb\n";
    let error_message = "Panic condition met!";
    let span = ast::Span { start: Position { line: 0, column: 0 }, end: Position { line: 2, column: 1 } };

    let formatter = Formatter {
        pattern,
        err: &error_message,
        span: &span,
        aux_span: None,
    };

    let mut output = String::new();
    let _ = write!(&mut output, "{}", formatter);
}

