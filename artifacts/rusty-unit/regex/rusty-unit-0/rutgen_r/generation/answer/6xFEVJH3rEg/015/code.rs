// Answer 0

#[test]
fn test_fmt_no_newline_error() {
    use std::fmt;

    struct TestError {
        err: String,
        pattern: String,
    }

    impl TestError {
        fn new(err: &str, pattern: &str) -> Self {
            TestError {
                err: err.to_string(),
                pattern: pattern.to_string(),
            }
        }
    }

    impl fmt::Debug for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Debug: {}", self.pattern)
        }
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Display: {}", self.pattern)
        }
    }

    struct Spans {
        multi_line: Vec<LineSpan>,
    }

    impl Spans {
        fn from_formatter(error: &TestError) -> Self {
            Spans {
                multi_line: Vec::new(),
            }
        }

        fn notate(&self) -> String {
            "Notated information about the spans.".to_string()
        }
    }

    struct LineSpan {
        start: Position,
        end: Position,
    }

    struct Position {
        line: usize,
        column: usize,
    }

    let mut output = Vec::new();
    let error = TestError::new("some error occurred", "a*b+c");
    let spans = Spans::from_formatter(&error);
    let result = {
        let result = writeln!(output, "regex parse error:");
        let notated = spans.notate();
        let result = write!(output, "{}", notated);
        let result = write!(output, "error: {}", error.err);
        result
    };

    assert!(result.is_err());
    assert_eq!(String::from_utf8_lossy(&output), "regex parse error:\nNotated information about the spans.error: some error occurred");
}

