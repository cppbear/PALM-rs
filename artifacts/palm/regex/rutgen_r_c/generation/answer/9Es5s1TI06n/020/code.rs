// Answer 0

#[test]
fn test_fmt_capture_limit_exceeded() {
    let error = ErrorKind::CaptureLimitExceeded;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, format!("exceeded the maximum number of capturing groups ({})", ::std::u32::MAX));
}

#[test]
fn test_fmt_class_escape_invalid() {
    let error = ErrorKind::ClassEscapeInvalid;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "invalid escape sequence found in character class");
}

#[test]
fn test_fmt_class_range_invalid() {
    let error = ErrorKind::ClassRangeInvalid;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "invalid character class range, the start must be <= the end");
}

#[test]
fn test_fmt_class_range_literal() {
    let error = ErrorKind::ClassRangeLiteral;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "invalid range boundary, must be a literal");
}

#[test]
fn test_fmt_class_unclosed() {
    let error = ErrorKind::ClassUnclosed;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "unclosed character class");
}

#[test]
fn test_fmt_decimal_empty() {
    let error = ErrorKind::DecimalEmpty;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "decimal literal empty");
}

#[test]
fn test_fmt_escape_unexpected_eof() {
    let error = ErrorKind::EscapeUnexpectedEof;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "incomplete escape sequence, reached end of pattern prematurely");
}

#[test]
fn test_fmt_flag_unrecognized() {
    let error = ErrorKind::FlagUnrecognized;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "unrecognized flag");
}

#[test]
fn test_fmt_group_name_empty() {
    let error = ErrorKind::GroupNameEmpty;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "empty capture group name");
}

#[test]
fn test_fmt_nest_limit_exceeded() {
    let limit = 5u32; // example limit
    let error = ErrorKind::NestLimitExceeded(limit);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, format!("exceed the maximum number of nested parentheses/brackets ({})", limit));
}

