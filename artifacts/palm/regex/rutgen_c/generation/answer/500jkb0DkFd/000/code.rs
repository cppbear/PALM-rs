// Answer 0

#[test]
fn test_description_unicode_not_allowed() {
    let error_kind = ErrorKind::UnicodeNotAllowed;
    let error = Error {
        kind: error_kind,
        pattern: String::from("(?-u:\\pL)"),
        span: Span { start: Position::from(0), end: Position::from(12) },
    };
    assert_eq!(error.description(), "Unicode not allowed here");
}

#[test]
fn test_description_invalid_utf8() {
    let error_kind = ErrorKind::InvalidUtf8;
    let error = Error {
        kind: error_kind,
        pattern: String::from("invalid utf-8 pattern"),
        span: Span { start: Position::from(0), end: Position::from(25) },
    };
    assert_eq!(error.description(), "pattern can match invalid UTF-8");
}

#[test]
fn test_description_unicode_property_not_found() {
    let error_kind = ErrorKind::UnicodePropertyNotFound;
    let error = Error {
        kind: error_kind,
        pattern: String::from("\\p{UnknownProperty}"),
        span: Span { start: Position::from(0), end: Position::from(22) },
    };
    assert_eq!(error.description(), "Unicode property not found");
}

#[test]
fn test_description_unicode_property_value_not_found() {
    let error_kind = ErrorKind::UnicodePropertyValueNotFound;
    let error = Error {
        kind: error_kind,
        pattern: String::from("\\p{Property=UnknownValue}"),
        span: Span { start: Position::from(0), end: Position::from(30) },
    };
    assert_eq!(error.description(), "Unicode property value not found");
}

#[test]
fn test_description_empty_class_not_allowed() {
    let error_kind = ErrorKind::EmptyClassNotAllowed;
    let error = Error {
        kind: error_kind,
        pattern: String::from("[]"),
        span: Span { start: Position::from(0), end: Position::from(2) },
    };
    assert_eq!(error.description(), "empty character classes are not allowed");
}

