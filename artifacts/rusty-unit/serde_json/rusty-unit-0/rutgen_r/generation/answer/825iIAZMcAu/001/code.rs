// Answer 0

#[test]
fn test_custom_with_string() {
    let msg = "This is a test message".to_string();
    let error = custom(msg);
    assert_eq!(error.to_string(), "Error: This is a test message");
}

#[test]
fn test_custom_with_integer() {
    let msg = 404; 
    let error = custom(msg);
    assert_eq!(error.to_string(), "Error: 404");
}

#[test]
fn test_custom_with_float() {
    let msg = 3.14; 
    let error = custom(msg);
    assert_eq!(error.to_string(), "Error: 3.14");
}

#[test]
fn test_custom_with_char() {
    let msg = 'A'; 
    let error = custom(msg);
    assert_eq!(error.to_string(), "Error: A");
}

#[test]
fn test_custom_with_empty_string() {
    let msg = ""; 
    let error = custom(msg);
    assert_eq!(error.to_string(), "Error: ");
}

#[should_panic]
fn test_custom_with_panic() {
    let msg = vec![1, 2, 3]; // vector does not implement Display
    let _error = custom(msg);
}

