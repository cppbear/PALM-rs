// Answer 0

#[test]
fn test_invalid_byte_display() {
    let error = DecodeError::InvalidByte(4, b'!');
    let result = format!("{}", error);
    assert_eq!(result, "Invalid symbol 33, offset 4.");
}

#[test]
fn test_invalid_length_display() {
    let error = DecodeError::InvalidLength(5);
    let result = format!("{}", error);
    assert_eq!(result, "Invalid input length: 5");
}

#[test]
fn test_invalid_last_symbol_display() {
    let error = DecodeError::InvalidLastSymbol(6, b'&');
    let result = format!("{}", error);
    assert_eq!(result, "Invalid last symbol 38, offset 6.");
}

#[test]
fn test_invalid_padding_display() {
    let error = DecodeError::InvalidPadding;
    let result = format!("{}", error);
    assert_eq!(result, "Invalid padding");
}

