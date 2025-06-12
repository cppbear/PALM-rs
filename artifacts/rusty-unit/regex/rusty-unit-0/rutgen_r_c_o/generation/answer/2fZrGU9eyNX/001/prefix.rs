// Answer 0

#[test]
fn test_auxiliary_span_invalid_utf8() {
    let error = Error {
        kind: ErrorKind::InvalidUtf8,
        pattern: String::from("invalid utf8"),
        span: Span {
            start: Position::new(0),
            end: Position::new(10),
        },
    };
    let _ = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_unicode_property_not_found() {
    let error = Error {
        kind: ErrorKind::UnicodePropertyNotFound,
        pattern: String::from("unrecognized property"),
        span: Span {
            start: Position::new(5),
            end: Position::new(15),
        },
    };
    let _ = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_empty_class_not_allowed() {
    let error = Error {
        kind: ErrorKind::EmptyClassNotAllowed,
        pattern: String::from("empty class"),
        span: Span {
            start: Position::new(2),
            end: Position::new(12),
        },
    };
    let _ = error.auxiliary_span();
}

#[test]
fn test_auxiliary_span_unicode_feature_not_allowed() {
    let error = Error {
        kind: ErrorKind::UnicodeNotAllowed,
        pattern: String::from("unicode feature"),
        span: Span {
            start: Position::new(8),
            end: Position::new(20),
        },
    };
    let _ = error.auxiliary_span();
}

