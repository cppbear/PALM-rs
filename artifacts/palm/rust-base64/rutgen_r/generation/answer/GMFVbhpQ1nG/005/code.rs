// Answer 0

#[test]
fn test_new_valid_alphabet() {
    // Arrange
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    // Act
    let result = base64::new(alphabet);
    
    // Assert
    assert!(result.is_ok());
}

#[test]
fn test_new_invalid_length() {
    // Arrange
    let alphabet = "ABC"; // Invalid length
    
    // Act
    let result = base64::new(alphabet);
    
    // Assert
    assert!(result.is_err());
}

#[test]
fn test_new_unprintable_byte() {
    // Arrange
    let alphabet = "ABCDEFGHIKLMNOPQRSTUVWXYYZabcdefghijklmnopqrstuvwxyz0123456789+/\x00"; // Contains unprintable (null byte)
    
    // Act
    let result = base64::new(alphabet);
    
    // Assert
    assert!(result.is_err());
}

#[test]
fn test_new_reserved_byte() {
    // Arrange
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    // Act
    let result = base64::new(alphabet.replace("=", "A").as_str());
    
    // Assert
    assert!(result.is_ok());
}

#[test]
fn test_new_duplicated_byte() {
    // Arrange
    let alphabet = "ABCDEFGHIKLMNOPQRSTUVWXYYZabcdefghijklmnopqrstuvwxyz0123456789+AA"; // Duplicated 'A'
    
    // Act
    let result = base64::new(alphabet);
    
    // Assert
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_new_boundary_condition_min_byte() {
    // Arrange
    let alphabet = " !" ; // Only two bytes, should trigger invalid length
    
    // Act
    let _ = base64::new(alphabet);
}

#[test]
#[should_panic]
fn test_new_boundary_condition_max_byte() {
    // Arrange
    let alphabet = "A".repeat(65); // Exceeds maximum alphabet size
    
    // Act
    let _ = base64::new(alphabet.as_str());
}

