// Answer 0

#[test]
fn test_error_kind_escape_hex_invalid_display() {
    use std::fmt::Write;

    let error = ErrorKind::EscapeHexInvalid;
    let mut output = String::new();
    let result = error.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "hexadecimal literal is not a Unicode scalar value");
}

#[test]
fn test_error_kind_escape_hex_invalid_digit_display() {
    use std::fmt::Write;

    let error = ErrorKind::EscapeHexInvalidDigit;
    let mut output = String::new();
    let result = error.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "invalid hexadecimal digit");
}

