// Answer 0

#[test]
fn fmt_single_line_error() {
    struct TestError;

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "test error")
        }
    }

    let pattern = "abc";
    let span = ast::Span {
        start: Position { line: 0, column: 0 },
        end: Position { line: 0, column: 3 },
    };

    let formatter = Formatter {
        pattern,
        err: &TestError,
        span: &span,
        aux_span: None,
    };

    let mut output = Vec::new();
    let result = writeln!(output, "{}", formatter);
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "regex parse error:\n    abc\nerror: test error\n");
}

#[test]
fn fmt_multi_line_error() {
    struct TestError;

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "test error")
        }
    }

    let pattern = "abc\ndef";
    let span = ast::Span {
        start: Position { line: 0, column: 0 },
        end: Position { line: 1, column: 3 },
    };

    let multi_line_span = ast::Span {
        start: Position { line: 0, column: 1 },
        end: Position { line: 1, column: 2 },
    };

    let formatter = Formatter {
        pattern,
        err: &TestError,
        span: &span,
        aux_span: Some(&multi_line_span),
    };

    let mut output = Vec::new();
    let result = writeln!(output, "{}", formatter);
    
    assert!(result.is_ok());
    assert!(String::from_utf8(output).unwrap().contains("multi line"));
    assert!(String::from_utf8(output).unwrap().contains("error: test error"));
}

