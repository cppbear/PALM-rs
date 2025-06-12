// Answer 0

#[test]
fn test_formatter_single_line() {
    use std::fmt;

    struct DummyError;

    impl fmt::Display for DummyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "dummy error")
        }
    }

    let pattern = "a+b*c";
    let error = DummyError;
    let span = ast::Span {
        start: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 5 },
    };

    let formatter = Formatter {
        pattern,
        err: &error,
        span: &span,
        aux_span: None,
    };

    let mut output = Vec::new();
    let result = formatter.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "regex parse error:\n\
        a+b*c\n\
        error: dummy error"
    );
}

#[test]
fn test_formatter_single_line_with_multi_line_span() {
    use std::fmt;

    struct DummyError;

    impl fmt::Display for DummyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "dummy error")
        }
    }

    let pattern = "a+b*c";
    let error = DummyError;
    let span = ast::Span {
        start: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 5 },
    };

    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line: vec![vec![span.clone()]], // Simulating a line with a span
        multi_line: vec![], // No multi-line spans
    };

    let mut output = Vec::new();
    let result = spans.notate(); // This would simulate a situation when the spans are associated with the formatter.

    assert!(result.is_ok());
}

