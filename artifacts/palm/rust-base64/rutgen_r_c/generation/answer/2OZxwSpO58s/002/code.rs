// Answer 0

#[test]
fn test_invalid_byte_display() {
    let error = DecodeError::InvalidByte(5, b'!');
    let mut output = String::new();
    let result = write!(output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid symbol 33, offset 5.");
}

#[test]
fn test_invalid_length_display() {
    let error = DecodeError::InvalidLength(3);
    let mut output = String::new();
    let result = write!(output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid input length: 3");
}

#[test]
fn test_invalid_last_symbol_display() {
    let error = DecodeError::InvalidLastSymbol(7, b'@');
    let mut output = String::new();
    let result = write!(output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid last symbol 64, offset 7.");
}

#[test]
fn test_invalid_padding_display() {
    let error = DecodeError::InvalidPadding;
    let mut output = String::new();
    let result = write!(output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid padding");
}

