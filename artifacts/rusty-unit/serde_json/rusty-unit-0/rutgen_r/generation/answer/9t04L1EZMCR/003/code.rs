// Answer 0

#[test]
fn test_pointer_mut_invalid_pointer_empty() {
    use serde_json::Value;
    let mut value = Value::Object(serde_json::Map::new());

    // Test with an empty pointer
    let result = value.pointer_mut("");
    assert_eq!(result, Some(&mut value));
}

#[test]
fn test_pointer_mut_invalid_pointer_no_start_slash() {
    use serde_json::Value;
    let mut value = Value::Object(serde_json::Map::new());

    // Test with a pointer that does not start with '/'
    let result = value.pointer_mut("invalid/pointer");
    assert_eq!(result, None);
}

