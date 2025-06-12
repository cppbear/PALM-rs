// Answer 0

#[test]
fn test_unit_variant_success() {
    // Given the method `unit_variant` that returns `Ok(())`
    
    // Act
    let result = unit_variant();
    
    // Assert
    assert!(result.is_ok());
}

