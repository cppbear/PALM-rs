// Answer 0

#[test]
fn test_error_pattern() {
    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
        pattern: String::from("abc(def)"),
        span: Span { start: Position(0), end: Position(9) },
    };
    
    assert_eq!(error.pattern(), "abc(def)");
}

#[test]
fn test_error_pattern_empty() {
    let error = Error {
        kind: ErrorKind::ClassEscapeInvalid,
        pattern: String::from(""),
        span: Span { start: Position(0), end: Position(0) },
    };
    
    assert_eq!(error.pattern(), "");
}

