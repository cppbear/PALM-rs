// Answer 0

#[test]
fn test_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_eof_while_parsing_value_edge_case() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

