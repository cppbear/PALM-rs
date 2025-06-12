// Answer 0

#[test]
fn test_json_value_display_compact_format() {
    use serde_json::{json, Value};

    let json_value = json!({"city": "London", "street": "10 Downing Street"});
    
    let compact_format = format!("{}", json_value);
    assert_eq!(compact_format, "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
}

#[test]
fn test_json_value_display_compact_format_with_null() {
    use serde_json::{json, Value};

    let json_value = json!(null);

    let compact_format = format!("{}", json_value);
    assert_eq!(compact_format, "null");
}

#[test]
fn test_json_value_display_compact_format_with_bool() {
    use serde_json::{json, Value};

    let json_value = json!(true);
    
    let compact_format = format!("{}", json_value);
    assert_eq!(compact_format, "true");
}

#[test]
fn test_json_value_display_compact_format_with_number() {
    use serde_json::{json, Value};

    let json_value = json!(12.5);
    
    let compact_format = format!("{}", json_value);
    assert_eq!(compact_format, "12.5");
}

#[test]
fn test_json_value_display_compact_format_with_string() {
    use serde_json::{json, Value};

    let json_value = json!("a string");

    let compact_format = format!("{}", json_value);
    assert_eq!(compact_format, "\"a string\"");
}

#[test]
fn test_json_value_display_compact_format_with_array() {
    use serde_json::{json, Value};

    let json_value = json!(["an", "array"]);
    
    let compact_format = format!("{}", json_value);
    assert_eq!(compact_format, "[\"an\",\"array\"]");
}

#[test]
fn test_json_value_display_compact_format_with_object() {
    use serde_json::{json, Value};

    let json_value = json!({ "an": "object" });

    let compact_format = format!("{}", json_value);
    assert_eq!(compact_format, "{\"an\":\"object\"}");
}

