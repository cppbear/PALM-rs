// Answer 0

#[test]
fn test_error_kind() {
    let error_kind = ErrorKind::CaptureLimitExceeded;
    let error = Error {
        kind: error_kind,
        pattern: String::from("test pattern"),
        span: Span {
            start: Position(0),
            end: Position(10),
        },
    };
    
    assert_eq!(error.kind(), &error_kind);
}

#[test]
fn test_error_kind_unicode_not_allowed() {
    let error_kind = ErrorKind::UnicodeNotAllowed;
    let error = Error {
        kind: error_kind,
        pattern: String::from("unicode test"),
        span: Span {
            start: Position(5),
            end: Position(11),
        },
    };
    
    assert_eq!(error.kind(), &error_kind);
}

#[test]
fn test_error_kind_empty_class_not_allowed() {
    let error_kind = ErrorKind::EmptyClassNotAllowed;
    let error = Error {
        kind: error_kind,
        pattern: String::from("[]"),
        span: Span {
            start: Position(0),
            end: Position(2),
        },
    };
    
    assert_eq!(error.kind(), &error_kind);
}

