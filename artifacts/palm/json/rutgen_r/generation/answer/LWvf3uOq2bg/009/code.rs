// Answer 0

#[derive(Debug)]
enum ErrorCode {
    ControlCharacterWhileParsingString,
    // other variants omitted for brevity
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorCode::ControlCharacterWhileParsingString => {
                f.write_str("control character (\\u0000-\\u001F) found while parsing a string")
            },
            // other variants omitted for brevity
        }
    }
}

#[test]
fn test_control_character_error() {
    let error = ErrorCode::ControlCharacterWhileParsingString;
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "control character (\\u0000-\\u001F) found while parsing a string");
}

