// Answer 0

#[test]
fn test_error_code_fmt_message() {
    let msg = Box::<str>::from("A simple error message");
    let error_code = ErrorCode::Message(msg);
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "A simple error message");
}

#[test]
fn test_error_code_fmt_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_fmt_number_out_of_range() {
    let error_code = ErrorCode::NumberOutOfRange;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "number out of range");
}

#[test]
fn test_error_code_fmt_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "invalid number");
} 

#[test]
fn test_error_code_fmt_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "EOF while parsing an object");
}

#[test]
fn test_error_code_fmt_control_character_while_parsing_string() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut output = String::new();
    let result = write!(output, "{}", error_code);
    assert!(result.is_ok());
    assert_eq!(output, "control character (\\u0000-\\u001F) found while parsing a string");
}

