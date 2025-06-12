// Answer 0

fn test_error_code_message() {
    struct LocalErrorImpl;
    let message = Box::from("This is a test error message".into_boxed_str());
    let error_code = ErrorCode::Message(message);
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "This is a test error message");
}

fn test_error_code_io() {
    struct LocalErrorImpl;
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "IO error occurred");
    let error_code = ErrorCode::Io(io_error);
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "IO error occurred");
}

fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing a list");
}

fn test_error_code_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "EOF while parsing an object");
}

fn test_error_code_lone_leading_surrogate_in_hex_escape() {
    let error_code = ErrorCode::LoneLeadingSurrogateInHexEscape;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "lone leading surrogate in hex escape");
}

fn test_error_code_trailing_comma() {
    let error_code = ErrorCode::TrailingComma;
    let mut buffer = String::new();
    let result = error_code.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "trailing comma");
}

