// Answer 0

#[test]
fn test_formatter_multi_line_error() {
    struct MockError;

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error message")
        }
    }

    let pattern = "some\nmultiline\npattern";
    let span = ast::Span {
        start: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 15 },
    };
    
    let formatter = Formatter {
        pattern,
        err: &MockError,
        span: &span,
        aux_span: None,
    };

    let result = formatter.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

#[test]
fn test_formatter_single_line_error() {
    struct MockError;

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error message")
        }
    }

    let pattern = "single line pattern";
    let span = ast::Span {
        start: Position { line: 0, column: 0 },
        end: Position { line: 0, column: 21 },
    };
    
    let formatter = Formatter {
        pattern,
        err: &MockError,
        span: &span,
        aux_span: None,
    };

    let result = formatter.fmt(&mut std::fmt::Formatter::new());
    assert!(result.is_ok());
}

