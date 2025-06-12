// Answer 0

#[test]
fn test_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_message() {
    let error_code = ErrorCode::Message(Box::from("test message"));
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

