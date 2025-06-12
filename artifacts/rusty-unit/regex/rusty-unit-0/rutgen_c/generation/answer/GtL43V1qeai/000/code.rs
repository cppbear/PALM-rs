// Answer 0

#[test]
fn test_error_kind() {
    use std::string::String;

    let error_span = Span { start: 0, end: 5 };
    let error_pattern = String::from("example");
    
    let error = Error {
        kind: ErrorKind::UnicodeNotAllowed,
        pattern: error_pattern,
        span: error_span,
    };

    assert_eq!(error.kind(), &ErrorKind::UnicodeNotAllowed);
}

#[test]
fn test_error_kind_invalid_utf8() {
    use std::string::String;

    let error_span = Span { start: 10, end: 15 };
    let error_pattern = String::from("invalid utf8");

    let error = Error {
        kind: ErrorKind::InvalidUtf8,
        pattern: error_pattern,
        span: error_span,
    };

    assert_eq!(error.kind(), &ErrorKind::InvalidUtf8);
}

#[test]
fn test_error_kind_unicode_property_not_found() {
    use std::string::String;

    let error_span = Span { start: 20, end: 25 };
    let error_pattern = String::from("property not found");

    let error = Error {
        kind: ErrorKind::UnicodePropertyNotFound,
        pattern: error_pattern,
        span: error_span,
    };

    assert_eq!(error.kind(), &ErrorKind::UnicodePropertyNotFound);
}

#[test]
fn test_error_kind_empty_class_not_allowed() {
    use std::string::String;

    let error_span = Span { start: 30, end: 35 };
    let error_pattern = String::from("empty class");

    let error = Error {
        kind: ErrorKind::EmptyClassNotAllowed,
        pattern: error_pattern,
        span: error_span,
    };

    assert_eq!(error.kind(), &ErrorKind::EmptyClassNotAllowed);
}

