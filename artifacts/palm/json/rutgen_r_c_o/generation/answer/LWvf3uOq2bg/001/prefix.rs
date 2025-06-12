// Answer 0

#[test]
fn test_fmt_recursion_limit_exceeded() {
    let error_code = ErrorCode::RecursionLimitExceeded;
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

#[test]
fn test_fmt_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_trailing_characters() {
    let error_code = ErrorCode::TrailingCharacters;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

