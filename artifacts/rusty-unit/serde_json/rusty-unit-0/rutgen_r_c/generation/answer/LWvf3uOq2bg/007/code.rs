// Answer 0

#[test]
fn test_error_code_fmt_message() {
    let msg = Box::<str>::from("An error occurred");
    let error_code = ErrorCode::Message(msg);
    let mut output = String::new();
    write!(&mut output, "{}", error_code).unwrap();
    assert_eq!(output, "An error occurred");
}

#[test]
fn test_error_code_fmt_io_error() {
    let io_error = io::Error::new(ErrorKind::Other, "IO Error");
    let error_code = ErrorCode::Io(io_error);
    let mut output = String::new();
    write!(&mut output, "{}", error_code).unwrap();
    assert_eq!(output, "IO Error");
}

#[test]
fn test_error_code_fmt_eof_while_parsing_list() {
    let error_code = ErrorCode::EofWhileParsingList;
    let mut output = String::new();
    write!(&mut output, "{}", error_code).unwrap();
    assert_eq!(output, "EOF while parsing a list");
}

#[test]
fn test_error_code_fmt_expected_numeric_key() {
    let error_code = ErrorCode::ExpectedNumericKey;
    let mut output = String::new();
    write!(&mut output, "{}", error_code).unwrap();
    assert_eq!(output, "invalid value: expected key to be a number in quotes");
}

#[test]
fn test_error_code_fmt_float_key_must_be_finite() {
    let error_code = ErrorCode::FloatKeyMustBeFinite;
    let mut output = String::new();
    write!(&mut output, "{}", error_code).unwrap();
    assert_eq!(output, "float key must be finite (got NaN or +/-inf)");
}

