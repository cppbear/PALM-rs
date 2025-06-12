// Answer 0

#[test]
fn test_fmt_expected_some_ident() {
    let error_code = ErrorCode::ExpectedSomeIdent;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_list_comma_or_end() {
    let error_code = ErrorCode::ExpectedListCommaOrEnd;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_object_comma_or_end() {
    let error_code = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

