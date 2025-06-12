// Answer 0

#[test]
fn test_error_pattern() {
    let error = Error {
        kind: ErrorKind::UnicodeNotAllowed,
        pattern: String::from("(?-u:\\pL)"),
        span: Span { start: Position::from(0), end: Position::from(10) },
    };
    assert_eq!(error.pattern(), "(?-u:\\pL)");
}

#[test]
fn test_error_pattern_empty() {
    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
        pattern: String::new(),
        span: Span { start: Position::from(0), end: Position::from(0) },
    };
    assert_eq!(error.pattern(), "");
}

#[test]
fn test_error_pattern_special_characters() {
    let error = Error {
        kind: ErrorKind::EscapeUnrecognized,
        pattern: String::from(".*[\\d]"),
        span: Span { start: Position::from(0), end: Position::from(6) },
    };
    assert_eq!(error.pattern(), ".*[\\d]");
}

#[test]
fn test_error_kind() {
    let error = Error {
        kind: ErrorKind::ClassRangeInvalid,
        pattern: String::from("[a-z]"),
        span: Span { start: Position::from(0), end: Position::from(5) },
    };
    assert_eq!(error.kind(), &ErrorKind::ClassRangeInvalid);
}

