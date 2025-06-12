// Answer 0

#[test]
fn test_key_must_be_a_string() {
    // Arrange
    let error = key_must_be_a_string();
    
    // Assert
    assert_eq!(error.err.code, ErrorCode::KeyMustBeAString);
    assert_eq!(error.err.line, 0);
    assert_eq!(error.err.column, 0);
}

#[test]
fn test_key_must_be_a_string_with_different_context() {
    // Arrange
    let error = key_must_be_a_string();
    
    // Assert
    assert!(matches!(error.err.code, ErrorCode::KeyMustBeAString));
    assert!(error.err.line == 0);
    assert!(error.err.column == 0);
}

