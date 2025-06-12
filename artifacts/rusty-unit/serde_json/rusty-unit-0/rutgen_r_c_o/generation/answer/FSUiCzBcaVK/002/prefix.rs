// Answer 0

#[test]
fn test_to_string_pretty_empty_object() {
    let value = &serde_json::json!({});
    let _ = to_string_pretty(value);
}

#[test]
fn test_to_string_pretty_empty_array() {
    let value = &serde_json::json!([]);
    let _ = to_string_pretty(value);
}

#[test]
fn test_to_string_pretty_simple_object() {
    let value = &serde_json::json!({"key": "value"});
    let _ = to_string_pretty(value);
}

#[test]
fn test_to_string_pretty_nested_object() {
    let value = &serde_json::json!({"key": {"nestedKey": "nestedValue"}});
    let _ = to_string_pretty(value);
}

#[test]
fn test_to_string_pretty_array() {
    let value = &serde_json::json!([1, 2, 3]);
    let _ = to_string_pretty(value);
}

#[test]
fn test_to_string_pretty_string_with_special_chars() {
    let value = &serde_json::json!(r#"{"text": "Line1\nLine2\tTabbed"}"#);
    let _ = to_string_pretty(value);
}

#[test]
fn test_to_string_pretty_valid_utf8_string() {
    let value = &serde_json::json!(r#"{"message": "Hello, 世界"}"#);
    let _ = to_string_pretty(value);
}

#[test]
fn test_to_string_pretty_long_string() {
    let value = &serde_json::json!(r#"{"longText": "A long text that has more than seventy characters, used for testing purposes."}"#);
    let _ = to_string_pretty(value);
}

#[test]
fn test_to_string_pretty_mixed_content() {
    let value = &serde_json::json!({"string": "text", "number": 123, "array": [true, false]});
    let _ = to_string_pretty(value);
}

