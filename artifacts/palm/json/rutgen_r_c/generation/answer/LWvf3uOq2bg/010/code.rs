// Answer 0

#[test]
fn test_error_code_display_message() {
    let error_message: Box<str> = "An error occurred".into();
    let error_code = ErrorCode::Message(error_message);
    let mut buf = String::new();
    let result = write!(&mut buf, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buf, "An error occurred");
}

#[test]
fn test_error_code_io_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "IO error occurred");
    let error_code = ErrorCode::Io(io_error);
    let mut buf = String::new();
    let result = write!(&mut buf, "{}", error_code);
    assert!(result.is_ok());
    assert!(buf.contains("IO error occurred"));
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut buf = String::new();
    let result = write!(&mut buf, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buf, "EOF while parsing a list");
}

#[test]
fn test_error_code_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut buf = String::new();
    let result = write!(&mut buf, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buf, "EOF while parsing an object");
}

#[test]
fn test_error_code_invalid_unicode_code_point() {
    let error_code = ErrorCode::InvalidUnicodeCodePoint;
    let mut buf = String::new();
    let result = write!(&mut buf, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(buf, "invalid unicode code point");
}

