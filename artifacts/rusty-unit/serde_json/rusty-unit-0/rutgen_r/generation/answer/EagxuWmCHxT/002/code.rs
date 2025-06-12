// Answer 0

#[test]
fn test_json_display_compact() {
    use serde_json::json;
    
    let json = json!({ "city": "London", "street": "10 Downing Street" });
    
    // Compact format:
    let compact = format!("{}", json);
    assert_eq!(compact, "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
}

#[test]
fn test_json_display_empty_object_compact() {
    use serde_json::json;
    
    let json = json!({});
    
    // Compact format of an empty JSON object:
    let compact = format!("{}", json);
    assert_eq!(compact, "{}");
}

#[test]
fn test_json_display_simple_value_compact() {
    use serde_json::json;
    
    let json = json!(42);
    
    // Compact format of a simple numeric value:
    let compact = format!("{}", json);
    assert_eq!(compact, "42");
}

#[test]
fn test_json_display_boolean_true_compact() {
    use serde_json::json;
    
    let json = json!(true);
    
    // Compact format of boolean true:
    let compact = format!("{}", json);
    assert_eq!(compact, "true");
}

#[test]
fn test_json_display_boolean_false_compact() {
    use serde_json::json;
    
    let json = json!(false);
    
    // Compact format of boolean false:
    let compact = format!("{}", json);
    assert_eq!(compact, "false");
}

// Test with special characters in the string
#[test]
fn test_json_display_special_characters_compact() {
    use serde_json::json;
    
    let json = json!({ "message": "Hello, \"world\"!\nNew line here." });
    
    // Compact format with special characters:
    let compact = format!("{}", json);
    assert_eq!(compact, "{\"message\":\"Hello, \\\"world\\\"!\\nNew line here.\"}");
}

