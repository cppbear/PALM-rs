// Answer 0

#[test]
fn test_error_code_unexpected_end_of_hex_escape_1() {
    let error_code = ErrorCode::UnexpectedEndOfHexEscape;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_error_code_unexpected_end_of_hex_escape_2() {
    let error_code = ErrorCode::UnexpectedEndOfHexEscape;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

