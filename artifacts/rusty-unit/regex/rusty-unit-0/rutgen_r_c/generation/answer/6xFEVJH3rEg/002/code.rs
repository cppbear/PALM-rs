// Answer 0

#[test]
fn test_formatter_fmt_with_newline() {
    struct MockError;

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    let pattern = "this is a test\nwith a newline";
    let span = ast::Span {
        start: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 15 }
    };
    let spans = &mut Spans {
        pattern,
        line_number_width: 0,
        by_line: vec![vec![span]],
        multi_line: vec![],
    };

    let formatter = Formatter {
        pattern,
        err: &MockError,
        span: &span,
        aux_span: None,
    };

    let result = fmt::format(formatter.fmt(&mut String::new()));
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_formatter_fmt_with_divider_error() {
    struct MockError;

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    let pattern = "this is a test\nwith a newline";
    let span = ast::Span {
        start: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 15 }
    };

    let formatter = Formatter {
        pattern,
        err: &MockError,
        span: &span,
        aux_span: None,
    };

    let result = formatter.fmt(&mut String::from("error occurred"));
    // Expecting a panic due to the `writeln!(f, "{}", divider)?` error condition
    assert!(result.is_err());
}

