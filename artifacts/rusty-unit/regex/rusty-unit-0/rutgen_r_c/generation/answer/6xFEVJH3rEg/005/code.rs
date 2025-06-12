// Answer 0

fn test_formatter_display_with_multi_line_error() {
    struct MockError;
    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    let pattern = "a\nb\nc"; // Multi-line pattern
    let err = MockError;
    let span = &ast::Span {
        start: Position { line: 1, column: 1 },
        end: Position { line: 1, column: 2 },
    };
    let formatter = Formatter {
        pattern,
        err: &err,
        span,
        aux_span: None,
    };

    let mut output = Vec::new();
    {
        let result = formatter.fmt(&mut output);
        assert!(result.is_ok()); // ensure fmt does not panic
    }

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("regex parse error:"));
    assert!(output_str.contains("~~"));
    assert!(output_str.contains("on line 1 (column 1) through line 1 (column 2)"));
    assert!(output_str.contains("error: mock error"));
}

fn test_formatter_display_without_multi_line_error() {
    struct MockError;
    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    let pattern = "abc"; // Single-line pattern
    let err = MockError;
    let span = &ast::Span {
        start: Position { line: 1, column: 1 },
        end: Position { line: 1, column: 4 },
    };
    let formatter = Formatter {
        pattern,
        err: &err,
        span,
        aux_span: None,
    };

    let mut output = Vec::new();
    {
        let result = formatter.fmt(&mut output);
        assert!(result.is_ok()); // ensure fmt does not panic
    }

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("regex parse error:"));
    assert!(output_str.contains("abc"));
    assert!(output_str.contains("error: mock error"));
}

