// Answer 0

#[test]
fn test_error_code_invalid_unicode_code_point() {
    // Given
    struct MockError;

    impl std::fmt::Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Mock Error")
        }
    }

    enum ErrorCode {
        Message(&'static str),
        Io(MockError),
        InvalidUnicodeCodePoint,
        // Other variants omitted for brevity
    }

    // When
    let error = ErrorCode::InvalidUnicodeCodePoint;
    let mut output = String::new();
    let result = error.fmt(&mut std::fmt::Formatter::new(&mut output));
    
    // Then
    assert!(result.is_ok());
    assert_eq!(output, "invalid unicode code point");
}

#[test]
fn test_error_code_control_character_while_parsing_string() {
    // Given
    struct MockError;

    impl std::fmt::Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Mock Error")
        }
    }

    enum ErrorCode {
        Message(&'static str),
        Io(MockError),
        ControlCharacterWhileParsingString,
        // Other variants omitted for brevity
    }

    // When
    let error = ErrorCode::ControlCharacterWhileParsingString;
    let mut output = String::new();
    let result = error.fmt(&mut std::fmt::Formatter::new(&mut output));
    
    // Then
    assert!(result.is_ok());
    assert_eq!(output, "control character (\\u0000-\\u001F) found while parsing a string");
}

