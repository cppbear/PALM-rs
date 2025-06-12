// Answer 0

#[test]
fn test_fmt_capture_limit_exceeded() {
    use std::fmt::Write;

    let error_kind = ErrorKind::CaptureLimitExceeded;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, format!("exceeded the maximum number of capturing groups ({})", ::std::u32::MAX));
}

#[test]
fn test_fmt_class_escape_invalid() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassEscapeInvalid;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "invalid escape sequence found in character class");
}

#[test]
fn test_fmt_class_range_invalid() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassRangeInvalid;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "invalid character class range, the start must be <= the end");
}

#[test]
fn test_fmt_class_range_literal() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassRangeLiteral;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "invalid range boundary, must be a literal");
}

#[test]
fn test_fmt_class_unclosed() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassUnclosed;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "unclosed character class");
}

#[test]
fn test_fmt_escape_unrecognized() {
    use std::fmt::Write;

    let error_kind = ErrorKind::EscapeUnrecognized;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "unrecognized escape sequence");
}

