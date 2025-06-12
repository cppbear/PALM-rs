// Answer 0

#[test]
fn test_fmt_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing a list");
}

#[test]
fn test_fmt_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing an object");
}

#[test]
fn test_fmt_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing a string");
}

#[test]
fn test_fmt_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing a value");
}

