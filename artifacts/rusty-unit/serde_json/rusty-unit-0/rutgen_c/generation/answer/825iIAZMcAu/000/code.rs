// Answer 0

#[test]
fn test_custom_creates_error_with_message() {
    // Arrange
    let message = "Test error message";

    // Act
    let error = Error::custom(message);

    // Assert
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
    // Here we assume ErrorCode can be compared, and we check if it matches the expected message
    assert_eq!(error.err.code, ErrorCode::Message(message.into()));
}

#[test]
fn test_custom_creates_error_with_empty_message() {
    // Arrange
    let message = "";

    // Act
    let error = Error::custom(message);

    // Assert
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
    assert_eq!(error.err.code, ErrorCode::Message("".into()));
}

#[test]
#[should_panic]
fn test_custom_panics_on_invalid_input() {
    // Arrange
    let message = "\u{FFFF}";

    // Act
    let _error = Error::custom(message);
    // Expected to panic in parse_line_col, add an expect message if necessary
}

