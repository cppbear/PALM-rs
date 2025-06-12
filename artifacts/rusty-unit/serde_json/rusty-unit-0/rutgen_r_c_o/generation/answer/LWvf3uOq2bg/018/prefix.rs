// Answer 0

#[test]
fn test_fmt_expected_list_comma_or_end() {
    let error_code = ErrorCode::ExpectedListCommaOrEnd;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_object_comma_or_end() {
    let error_code = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

