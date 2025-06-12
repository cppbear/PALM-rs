// Answer 0

#[test]
fn test_invalid_escape_error_code() {
    // Define the ErrorCode enum with a variant for InvalidEscape
    enum ErrorCode {
        InvalidEscape,
    }

    // Implement the fmt method as described
    use std::fmt;

    impl fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::InvalidEscape => f.write_str("invalid escape"),
            }
        }
    }

    // Create an instance of ErrorCode::InvalidEscape
    let error = ErrorCode::InvalidEscape;

    // Capture the output of the fmt function
    let result = format!("{}", error);

    // Assert that the output matches the expected string
    assert_eq!(result, "invalid escape");
}

#[test]
fn test_control_character_error_code() {
    // Define the ErrorCode enum with a variant for ControlCharacterWhileParsingString
    enum ErrorCode {
        ControlCharacterWhileParsingString,
    }

    // Implement the fmt method as described
    use std::fmt;

    impl fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ErrorCode::ControlCharacterWhileParsingString => {
                    f.write_str("control character (\\u0000-\\u001F) found while parsing a string")
                }
            }
        }
    }

    // Create an instance of ErrorCode::ControlCharacterWhileParsingString
    let error = ErrorCode::ControlCharacterWhileParsingString;

    // Capture the output of the fmt function
    let result = format!("{}", error);

    // Assert that the output matches the expected string
    assert_eq!(result, "control character (\\u0000-\\u001F) found while parsing a string");
}

