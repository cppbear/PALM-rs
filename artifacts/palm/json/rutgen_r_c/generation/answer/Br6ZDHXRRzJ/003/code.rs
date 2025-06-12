// Answer 0

#[test]
fn test_unexpected_string() {
    use serde::de::Unexpected;
    
    let value = Value::String("test string".to_owned());
    
    let result = value.unexpected();
    
    match result {
        Unexpected::Str(s) => assert_eq!(s, "test string"),
        _ => panic!("Expected Unexpected::Str variant"),
    }
}

#[test]
fn test_unexpected_empty_string() {
    use serde::de::Unexpected;
    
    let value = Value::String("".to_owned());
    
    let result = value.unexpected();
    
    match result {
        Unexpected::Str(s) => assert_eq!(s, ""),
        _ => panic!("Expected Unexpected::Str variant"),
    }
}

#[test]
fn test_unexpected_special_characters_string() {
    use serde::de::Unexpected;
    
    let value = Value::String("!@#$%^&*()".to_owned());
    
    let result = value.unexpected();
    
    match result {
        Unexpected::Str(s) => assert_eq!(s, "!@#$%^&*()"),
        _ => panic!("Expected Unexpected::Str variant"),
    }
}

#[test]
fn test_unexpected_long_string() {
    use serde::de::Unexpected;
    
    let long_string = "a".repeat(1000);
    let value = Value::String(long_string.clone());
    
    let result = value.unexpected();
    
    match result {
        Unexpected::Str(s) => assert_eq!(s, long_string),
        _ => panic!("Expected Unexpected::Str variant"),
    }
}

