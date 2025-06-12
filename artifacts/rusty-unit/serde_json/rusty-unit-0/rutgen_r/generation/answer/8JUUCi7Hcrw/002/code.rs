// Answer 0

#[test]
fn test_to_vec_pretty_with_string() {
    let value = "Hello, world!";
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"\"Hello, world!\"\n");
}

#[test]
fn test_to_vec_pretty_with_numbers() {
    let value = 42;
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"42\n");
}

#[test]
fn test_to_vec_pretty_with_array() {
    let value = vec![1, 2, 3];
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"[\n  1,\n  2,\n  3\n]\n");
}

#[test]
fn test_to_vec_pretty_with_object() {
    let value = serde_json::json!({
        "key1": "value1",
        "key2": 2,
        "key3": [1, 2, 3],
    });
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"{\n  \"key1\": \"value1\",\n  \"key2\": 2,\n  \"key3\": [\n    1,\n    2,\n    3\n  ]\n}\n");
}

#[test]
fn test_to_vec_pretty_with_empty_object() {
    let value: serde_json::Value = serde_json::json!({});
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"{\n}\n");
}

