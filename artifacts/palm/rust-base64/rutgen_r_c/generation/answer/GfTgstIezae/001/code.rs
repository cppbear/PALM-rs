// Answer 0

#[test]
fn test_fmt_output_slice_too_small() {
    // Arrange
    let error_case = DecodeSliceError::OutputSliceTooSmall;

    // Act
    let result = format!("{}", error_case);

    // Assert
    assert_eq!(result, "Output slice too small");
}

#[test]
fn test_fmt_decode_error() {
    // Arrange
    let decode_error_case = DecodeError::InvalidByte(5, b'x');
    let error_case = DecodeSliceError::DecodeError(decode_error_case);

    // Act
    let result = format!("{}", error_case);

    // Assert
    assert!(result.contains("DecodeError: InvalidByte(5, 'x')"));
}

#[test]
fn test_fmt_with_invalid_length_error() {
    // Arrange
    let decode_error_case = DecodeError::InvalidLength(3);
    let error_case = DecodeSliceError::DecodeError(decode_error_case);

    // Act
    let result = format!("{}", error_case);

    // Assert
    assert!(result.contains("DecodeError: InvalidLength(3)"));
}

#[test]
fn test_fmt_with_invalid_last_symbol_error() {
    // Arrange
    let decode_error_case = DecodeError::InvalidLastSymbol(2, b'y');
    let error_case = DecodeSliceError::DecodeError(decode_error_case);

    // Act
    let result = format!("{}", error_case);

    // Assert
    assert!(result.contains("DecodeError: InvalidLastSymbol(2, 'y')"));
}

#[test]
fn test_fmt_with_invalid_padding_error() {
    // Arrange
    let decode_error_case = DecodeError::InvalidPadding;
    let error_case = DecodeSliceError::DecodeError(decode_error_case);

    // Act
    let result = format!("{}", error_case);

    // Assert
    assert!(result.contains("DecodeError: InvalidPadding"));
}

