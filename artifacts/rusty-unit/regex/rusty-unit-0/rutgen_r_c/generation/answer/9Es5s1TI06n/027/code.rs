// Answer 0

#[test]
fn test_errorkind_fmt_class_range_literal() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassRangeLiteral;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "invalid range boundary, must be a literal");
}

#[test]
fn test_errorkind_fmt_class_range_invalid() {
    use std::fmt::Write;

    let error_kind = ErrorKind::ClassRangeInvalid;
    let mut output = String::new();
    let result = error_kind.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "invalid character class range, \
                        the start must be <= the end");
}

