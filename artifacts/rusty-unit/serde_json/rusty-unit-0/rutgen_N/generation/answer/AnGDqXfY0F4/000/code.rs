// Answer 0

#[test]
fn test_from_str_valid_json() {
    let json_str = r#"{"key": "value"}"#;
    let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(json_str);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value["key"], "value");
}

#[test]
fn test_from_str_invalid_json() {
    let json_str = r#"{"key" "value"}"#; // missing colon
    let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(json_str);
    assert!(result.is_err());
}

#[test]
fn test_from_str_empty_string() {
    let json_str = r#""#; // empty string
    let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(json_str);
    assert!(result.is_err());
}

#[test]
fn test_from_str_array() {
    let json_str = r#"["value1", "value2"]"#;
    let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(json_str);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value[0], "value1");
    assert_eq!(value[1], "value2");
}

