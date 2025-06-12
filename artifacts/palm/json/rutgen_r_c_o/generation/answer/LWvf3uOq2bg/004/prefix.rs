// Answer 0

#[test]
fn test_fmt_trailing_comma() {
    let error_code = ErrorCode::TrailingComma;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_object() {
    let error_code = ErrorCode::EofWhileParsingObject;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_string() {
    let error_code = ErrorCode::EofWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_eof_while_parsing_value() {
    let error_code = ErrorCode::EofWhileParsingValue;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_colon() {
    let error_code = ErrorCode::ExpectedColon;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_list_comma_or_end() {
    let error_code = ErrorCode::ExpectedListCommaOrEnd;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_object_comma_or_end() {
    let error_code = ErrorCode::ExpectedObjectCommaOrEnd;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_some_ident() {
    let error_code = ErrorCode::ExpectedSomeIdent;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_some_value() {
    let error_code = ErrorCode::ExpectedSomeValue;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_double_quote() {
    let error_code = ErrorCode::ExpectedDoubleQuote;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_invalid_escape() {
    let error_code = ErrorCode::InvalidEscape;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_invalid_number() {
    let error_code = ErrorCode::InvalidNumber;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_number_out_of_range() {
    let error_code = ErrorCode::NumberOutOfRange;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_invalid_unicode_code_point() {
    let error_code = ErrorCode::InvalidUnicodeCodePoint;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_control_character_while_parsing_string() {
    let error_code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_key_must_be_a_string() {
    let error_code = ErrorCode::KeyMustBeAString;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_expected_numeric_key() {
    let error_code = ErrorCode::ExpectedNumericKey;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_float_key_must_be_finite() {
    let error_code = ErrorCode::FloatKeyMustBeFinite;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_lone_leading_surrogate_in_hex_escape() {
    let error_code = ErrorCode::LoneLeadingSurrogateInHexEscape;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_trailing_characters() {
    let error_code = ErrorCode::TrailingCharacters;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_unexpected_end_of_hex_escape() {
    let error_code = ErrorCode::UnexpectedEndOfHexEscape;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

#[test]
fn test_fmt_recursion_limit_exceeded() {
    let error_code = ErrorCode::RecursionLimitExceeded;
    let mut formatter = fmt::Formatter::new();
    error_code.fmt(&mut formatter);
}

