// Answer 0

#[test]
fn test_fmt_expected_numeric_key() {
    let error = ErrorCode::ExpectedNumericKey;
    let mut formatter = core::fmt::Formatter::new();
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_message() {
    let error_message = Box::<str>::from("This is an error message");
    let error = ErrorCode::Message(error_message);
    let mut formatter = core::fmt::Formatter::new();
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_list() {
    let error = ErrorCode::EofWhileParsingList;
    let mut formatter = core::fmt::Formatter::new();
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_object() {
    let error = ErrorCode::EofWhileParsingObject;
    let mut formatter = core::fmt::Formatter::new();
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_string() {
    let error = ErrorCode::EofWhileParsingString;
    let mut formatter = core::fmt::Formatter::new();
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_invalid_escape() {
    let error = ErrorCode::InvalidEscape;
    let mut formatter = core::fmt::Formatter::new();
    error.fmt(&mut formatter);
}

