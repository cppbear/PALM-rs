// Answer 0

#[test]
fn test_from_str_valid_json() {
    let json_input = r#"{"key": "value", "number": 123}"#;
    let deserializer = serde_json::from_str(json_input);
    assert!(deserializer.is_ok());
}

#[test]
fn test_from_str_empty_string() {
    let json_input = r#"{}"#;
    let deserializer = serde_json::from_str(json_input);
    assert!(deserializer.is_ok());
}

#[test]
#[should_panic]
fn test_from_str_invalid_json() {
    let json_input = r#"{key: value}"#; // Missing quotes
    let _deserializer = serde_json::from_str(json_input); // This should panic
}

#[test]
fn test_from_str_array_json() {
    let json_input = r#"[1, 2, 3]"#;
    let deserializer = serde_json::from_str(json_input);
    assert!(deserializer.is_ok());
}

#[test]
fn test_from_str_nested_json() {
    let json_input = r#"{"outer": {"inner": "value"}}"#;
    let deserializer = serde_json::from_str(json_input);
    assert!(deserializer.is_ok());
}

#[test]
#[should_panic]
fn test_from_str_non_json_string() {
    let json_input = "Not JSON";
    let _deserializer = serde_json::from_str(json_input); // This should panic
}

