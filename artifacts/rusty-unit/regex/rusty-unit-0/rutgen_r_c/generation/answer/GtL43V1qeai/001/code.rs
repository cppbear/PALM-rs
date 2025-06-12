// Answer 0

#[test]
fn test_kind_returns_unicode_not_allowed() {
    let error = Error {
        kind: ErrorKind::UnicodeNotAllowed,
        pattern: String::from("(?-u:\\pL)"),
        span: Span { start: Position::from(0), end: Position::from(10) },
    };
    assert_eq!(error.kind(), &ErrorKind::UnicodeNotAllowed);
}

#[test]
fn test_kind_returns_invalid_utf8() {
    let error = Error {
        kind: ErrorKind::InvalidUtf8,
        pattern: String::from("invalid utf8 pattern"),
        span: Span { start: Position::from(0), end: Position::from(25) },
    };
    assert_eq!(error.kind(), &ErrorKind::InvalidUtf8);
}

#[test]
fn test_kind_returns_unicode_property_not_found() {
    let error = Error {
        kind: ErrorKind::UnicodePropertyNotFound,
        pattern: String::from("\\p{NonExistentProperty}"),
        span: Span { start: Position::from(0), end: Position::from(30) },
    };
    assert_eq!(error.kind(), &ErrorKind::UnicodePropertyNotFound);
}

#[test]
fn test_kind_returns_empty_class_not_allowed() {
    let error = Error {
        kind: ErrorKind::EmptyClassNotAllowed,
        pattern: String::from("[]"),
        span: Span { start: Position::from(0), end: Position::from(2) },
    };
    assert_eq!(error.kind(), &ErrorKind::EmptyClassNotAllowed);
}

#[test]
fn test_kind_returns_capture_limit_exceeded() {
    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
        pattern: String::from("(.)(.)"),
        span: Span { start: Position::from(0), end: Position::from(5) },
    };
    assert_eq!(error.kind(), &ErrorKind::CaptureLimitExceeded);
}

