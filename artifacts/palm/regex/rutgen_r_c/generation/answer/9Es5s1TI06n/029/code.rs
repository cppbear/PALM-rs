// Answer 0

#[test]
fn test_error_kind_display_class_escape_invalid() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassEscapeInvalid;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "invalid escape sequence found in character class");
}

#[test]
fn test_error_kind_display_class_range_invalid() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassRangeInvalid;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "invalid character class range, the start must be <= the end");
}

#[test]
fn test_error_kind_display_class_range_literal() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassRangeLiteral;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "invalid range boundary, must be a literal");
}

#[test]
fn test_error_kind_display_class_unclosed() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassUnclosed;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "unclosed character class");
}

