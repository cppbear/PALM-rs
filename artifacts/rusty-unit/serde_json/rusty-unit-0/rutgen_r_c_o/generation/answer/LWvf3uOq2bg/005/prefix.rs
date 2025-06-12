// Answer 0

#[test]
fn test_lone_leading_surrogate_in_hex_escape() {
    let error_code = ErrorCode::LoneLeadingSurrogateInHexEscape;
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

#[test]
fn test_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

