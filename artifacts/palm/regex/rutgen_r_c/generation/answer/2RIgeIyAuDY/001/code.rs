// Answer 0

#[test]
fn test_error_pattern() {
    struct TestError {
        kind: ErrorKind,
        pattern: String,
        span: Span,
    }

    let error = TestError {
        kind: ErrorKind::UnicodeNotAllowed,
        pattern: String::from("(?-u:\\pL)"),
        span: Span { start: Position { pos: 0 }, end: Position { pos: 10 } },
    };

    assert_eq!(error.pattern(), "(?-u:\\pL)");
}

#[test]
fn test_error_pattern_empty_string() {
    struct TestError {
        kind: ErrorKind,
        pattern: String,
        span: Span,
    }

    let error = TestError {
        kind: ErrorKind::EmptyClassNotAllowed,
        pattern: String::from(""),
        span: Span { start: Position { pos: 0 }, end: Position { pos: 0 } },
    };

    assert_eq!(error.pattern(), "");
}

