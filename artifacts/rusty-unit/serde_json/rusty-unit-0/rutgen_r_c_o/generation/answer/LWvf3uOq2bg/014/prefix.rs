// Answer 0

#[test]
fn test_expected_double_quote() {
    let error_code = ErrorCode::ExpectedDoubleQuote;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_expected_double_quote_with_message() {
    let error_code = ErrorCode::Message(Box::from("Some error occurred"));
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

