// Answer 0

#[test]
fn test_error_code_invalid_escape() {
    let error_code = ErrorCode::InvalidEscape;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_control_character_while_parsing_string() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_number_out_of_range() {
    let error_code = ErrorCode::NumberOutOfRange;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

