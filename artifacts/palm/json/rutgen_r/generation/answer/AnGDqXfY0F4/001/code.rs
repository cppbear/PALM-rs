// Answer 0

#[test]
fn test_from_str_valid_json() {
    let json_str = r#"{"key": "value", "number": 42, "is_active": true}"#;
    let result = serde_json::de::from_str(json_str);
    assert!(result.is_ok());
    
    let value = result.unwrap();
    assert_eq!(value["key"], "value");
    assert_eq!(value["number"], 42);
    assert_eq!(value["is_active"], true);
}

#[test]
fn test_from_str_empty_string() {
    let json_str = r#"{}"#;
    let result = serde_json::de::from_str(json_str);
    assert!(result.is_ok());
    
    let value = result.unwrap();
    assert_eq!(value.as_object().unwrap().len(), 0);
}

#[test]
fn test_from_str_invalid_json() {
    let json_str = r#"{key: value}"#; // Invalid JSON (missing quotes)
    let result = serde_json::de::from_str(json_str);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_from_str_null_string() {
    let json_str: &str = std::ptr::null(); // Panic on null string
    let _ = serde_json::de::from_str(json_str);
}

#[test]
fn test_from_str_array_json() {
    let json_str = r#"["item1", "item2", "item3"]"#;
    let result = serde_json::de::from_str(json_str);
    assert!(result.is_ok());
    
    let value = result.unwrap();
    assert_eq!(value.as_array().unwrap().len(), 3);
}

#[test]
fn test_from_str_nested_json() {
    let json_str = r#"{"outer": {"inner": "value"}}"#;
    let result = serde_json::de::from_str(json_str);
    assert!(result.is_ok());
    
    let value = result.unwrap();
    assert_eq!(value["outer"]["inner"], "value");
}

#[test]
fn test_from_str_boolean_json() {
    let json_str = r#"{"is_valid": false}"#;
    let result = serde_json::de::from_str(json_str);
    assert!(result.is_ok());
    
    let value = result.unwrap();
    assert_eq!(value["is_valid"], false);
}

