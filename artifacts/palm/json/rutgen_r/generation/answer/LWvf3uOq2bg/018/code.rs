// Answer 0

#[test]
fn test_fmt_message() {
    struct ErrorCodeMessage;
    
    impl std::fmt::Display for ErrorCodeMessage {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("An example message")
        }
    }
    
    let error = ErrorCode::Message(ErrorCodeMessage);
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "An example message");
}

#[test]
fn test_fmt_eof_while_parsing_list() {
    let error = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_fmt_expected_list_comma_or_end() {
    let error = ErrorCode::ExpectedListCommaOrEnd;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `]`");
} 

#[test]
fn test_fmt_expected_object_comma_or_end() {
    let error = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "expected `,` or `}`");
} 

#[test]
fn test_fmt_control_character_while_parsing_string() {
    let error = ErrorCode::ControlCharacterWhileParsingString;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "control character (\\u0000-\\u001F) found while parsing a string");
} 

#[test]
fn test_fmt_trailing_comma() {
    let error = ErrorCode::TrailingComma;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "trailing comma");
} 

#[test]
fn test_fmt_trailing_characters() {
    let error = ErrorCode::TrailingCharacters;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "trailing characters");
} 

#[test]
fn test_fmt_unexpected_end_of_hex_escape() {
    let error = ErrorCode::UnexpectedEndOfHexEscape;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "unexpected end of hex escape");
}

#[test]
fn test_fmt_recursion_limit_exceeded() {
    let error = ErrorCode::RecursionLimitExceeded;
    let mut output = String::new();
    let result = error.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "recursion limit exceeded");
}

