// Answer 0

#[test]
fn test_fmt_display_parse_error() {
    let position_start = Position { value: 0 };
    let position_end = Position { value: 5 };
    let span = Span {
        start: position_start,
        end: position_end,
    };
    let error_kind = ErrorKind::CaptureLimitExceeded;
    let error = Error {
        kind: error_kind,
        pattern: String::from("test pattern"),
        span,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);

    assert!(result.is_ok());
    assert!(!output.is_empty());
}

#[test]
fn test_fmt_display_invalid_utf8_error() {
    let position_start = Position { value: 0 };
    let position_end = Position { value: 4 };
    let span = Span {
        start: position_start,
        end: position_end,
    };
    let error_kind = ErrorKind::InvalidUtf8;
    let error = Error {
        kind: error_kind,
        pattern: String::from("invalid utf8"),
        span,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);

    assert!(result.is_ok());
    assert!(!output.is_empty());
}

#[test]
fn test_fmt_display_empty_class_not_allowed_error() {
    let position_start = Position { value: 0 };
    let position_end = Position { value: 3 };
    let span = Span {
        start: position_start,
        end: position_end,
    };
    let error_kind = ErrorKind::EmptyClassNotAllowed;
    let error = Error {
        kind: error_kind,
        pattern: String::from("[]"),
        span,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);

    assert!(result.is_ok());
    assert!(!output.is_empty());
}

