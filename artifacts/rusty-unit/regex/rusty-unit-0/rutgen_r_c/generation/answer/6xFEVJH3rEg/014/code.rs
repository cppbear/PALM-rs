// Answer 0

#[test]
fn test_fmt_single_line_error() {
    struct MockError;
    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    struct MockSpan {
        start: Position,
        end: Position,
    }

    struct MockFormatter<'e> {
        pattern: &'e str,
        err: &'e MockError,
        span: &'e MockSpan,
        aux_span: Option<&'e MockSpan>,
    }

    impl<'e> MockFormatter<'e> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let spans = Spans::from_formatter(self);
            writeln!(f, "regex parse error:")?;
            let notated = spans.notate();
            write!(f, "{}", notated)?;
            write!(f, "error: {}", self.err)
        }
    }

    let error = MockError;
    let span = MockSpan {
        start: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 10 },
    };
    let formatter = MockFormatter {
        pattern: "abcde",
        err: &error,
        span: &span,
        aux_span: None,
    };

    let mut output = vec![];
    let result = formatter.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "regex parse error:\nerror: mock error");
}

#[test]
fn test_fmt_single_line_error_notate_err() {
    struct MockError;
    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    struct MockSpan {
        start: Position,
        end: Position,
    }

    struct MockFormatter<'e> {
        pattern: &'e str,
        err: &'e MockError,
        span: &'e MockSpan,
    }

    impl<'e> MockFormatter<'e> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let spans = Spans::from_formatter(self);
            writeln!(f, "regex parse error:")?;
            let notated = spans.notate();
            write!(f, "{}", notated)?;  // simulate an error here
            Err(fmt::Error)
        }
    }

    let error = MockError;
    let span = MockSpan {
        start: Position { line: 1, column: 0 },
        end: Position { line: 1, column: 10 },
    };
    let formatter = MockFormatter {
        pattern: "abcde",
        err: &error,
        span: &span,
    };

    let mut output = vec![];
    let result = formatter.fmt(&mut output);
    assert!(result.is_err());
}

