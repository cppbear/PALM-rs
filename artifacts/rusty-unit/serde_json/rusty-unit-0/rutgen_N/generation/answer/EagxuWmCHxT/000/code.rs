// Answer 0

#[test]
fn test_json_display_compact() {
    use serde_json::json;
    use std::fmt;

    // Construct a JSON value using the json! macro
    let json_value = json!({ "city": "London", "street": "10 Downing Street" });

    // Compact format
    let compact = format!("{}", json_value);
    assert_eq!(compact, "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
}

#[test]
fn test_json_display_pretty() {
    use serde_json::json;
    use std::fmt;

    // Construct a JSON value using the json! macro
    let json_value = json!({ "city": "London", "street": "10 Downing Street" });

    // Pretty format
    let pretty = format!("{:#}", json_value);
    assert_eq!(pretty, "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}");
}

