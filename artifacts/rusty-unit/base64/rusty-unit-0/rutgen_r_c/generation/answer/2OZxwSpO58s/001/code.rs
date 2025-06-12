// Answer 0

#[test]
fn test_decode_error_invalid_padding_display() {
    // Arrange
    let error = DecodeError::InvalidPadding;

    // Act
    let result = format!("{}", error);

    // Assert
    assert_eq!(result, "Invalid padding");
}

#[test]
fn test_decode_error_invalid_length_display() {
    // Arrange
    let error = DecodeError::InvalidLength(5);

    // Act
    let result = format!("{}", error);

    // Assert
    assert_eq!(result, "Invalid input length: 5");
}

#[test]
fn test_decode_error_invalid_byte_display() {
    // Arrange
    let error = DecodeError::InvalidByte(10, b'@');

    // Act
    let result = format!("{}", error);

    // Assert
    assert_eq!(result, "Invalid symbol 64, offset 10.");
}

#[test]
fn test_decode_error_invalid_last_symbol_display() {
    // Arrange
    let error = DecodeError::InvalidLastSymbol(6, b'=');

    // Act
    let result = format!("{}", error);

    // Assert
    assert_eq!(result, "Invalid last symbol 61, offset 6.");
}

