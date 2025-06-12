// Answer 0

#[test]
fn test_error_code_trailing_characters() {
    let error_code = ErrorCode::TrailingCharacters;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_message() {
    let error_code = ErrorCode::Message(Box::from("Trailing comma, valid input"));
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_invalid_escape() {
    let error_code = ErrorCode::InvalidEscape;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

