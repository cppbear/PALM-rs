// Answer 0

#[test]
fn test_error_code_message() {
    let error_message = Box::from("This is an error message");
    let error_code = ErrorCode::Message(error_message);

    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "This is an error message");
}

#[test]
fn test_error_code_io() {
    #[derive(Debug)]
    struct MockIoError;

    impl Display for MockIoError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock io error")
        }
    }

    let io_error = MockIoError;
    let error_code = ErrorCode::Io(io_error);

    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "mock io error");
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
fn test_error_code_float_key_must_be_finite() {
    let error_code = ErrorCode::FloatKeyMustBeFinite;

    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "float key must be finite (got NaN or +/-inf)");
}

#[test]
fn test_error_code_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;

    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "invalid number");
}

#[test]
fn test_error_code_invalid_unicode_code_point() {
    let error_code = ErrorCode::InvalidUnicodeCodePoint;

    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "invalid unicode code point");
}

#[test]
fn test_error_code_control_character_while_parsing_string() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;

    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "control character (\\u0000-\\u001F) found while parsing a string");
}

#[test]
fn test_error_code_recursion_limit_exceeded() {
    let error_code = ErrorCode::RecursionLimitExceeded;

    let mut output = String::new();
    let result = write!(&mut output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "recursion limit exceeded");
}

