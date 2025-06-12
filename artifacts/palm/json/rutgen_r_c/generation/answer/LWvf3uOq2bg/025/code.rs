// Answer 0

#[test]
fn test_error_code_message() {
    let long_message: Box<str> = "This is a long error message that exceeds typical lengths and is meant to test the formatting functionality.".into();
    let error_code = ErrorCode::Message(long_message);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);

    assert!(result.is_ok());
    assert_eq!(output, "This is a long error message that exceeds typical lengths and is meant to test the formatting functionality.");
}

#[test]
fn test_error_code_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);

    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);

    assert!(result.is_ok());
    assert_eq!(output, "expected `:`");
}

#[test]
fn test_error_code_io() {
    // Creating a simple IO error to test the IO variant
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "IO error occurred");
    let error_code = ErrorCode::Io(io_error);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);

    assert!(result.is_ok());
    assert_eq!(output, "IO error occurred");
}


