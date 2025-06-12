// Answer 0

#[test]
fn test_unexpected_with_bool() {
    // Arrange
    let value_true = Value::Bool(true);
    let value_false = Value::Bool(false);
    
    // Act
    let unexpected_true = value_true.unexpected();
    let unexpected_false = value_false.unexpected();
    
    // Assert
    match unexpected_true {
        Unexpected::Bool(b) => assert!(b, "Expected true but got false"),
        _ => panic!("Expected Unexpected::Bool for true value"),
    }
    
    match unexpected_false {
        Unexpected::Bool(b) => assert!(!b, "Expected false but got true"),
        _ => panic!("Expected Unexpected::Bool for false value"),
    }
}

