// Answer 0

#[test]
fn test_fmt_eof_while_parsing_string() {
    let error = ErrorCode::EofWhileParsingString;
    let mut formatter = fmt::Formatter::new(); 
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_list() {
    let error = ErrorCode::EofWhileParsingList;
    let mut formatter = fmt::Formatter::new(); 
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_object() {
    let error = ErrorCode::EofWhileParsingObject;
    let mut formatter = fmt::Formatter::new(); 
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_colon() {
    let error = ErrorCode::ExpectedColon;
    let mut formatter = fmt::Formatter::new(); 
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_some_value() {
    let error = ErrorCode::ExpectedSomeValue;
    let mut formatter = fmt::Formatter::new(); 
    error.fmt(&mut formatter);
}

#[test]
fn test_fmt_invalid_escape() {
    let error = ErrorCode::InvalidEscape;
    let mut formatter = fmt::Formatter::new(); 
    error.fmt(&mut formatter);
}

