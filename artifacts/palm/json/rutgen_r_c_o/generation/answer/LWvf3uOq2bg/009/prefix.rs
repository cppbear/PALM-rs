// Answer 0

#[test]
fn test_error_code_control_character_while_parsing_string() {
    let msg = Box::new(String::from("control character found"));
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_control_character_null() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_control_character_start_of_heading() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_control_character_start_of_text() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_control_character_end_of_text() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_control_character_expand_tab() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_control_character_end_of_transmission() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_control_character_pen() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

