// Answer 0

#[test]
fn test_to_vec_pretty_with_empty_string() {
    let value = "";
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"\"\""); // Pretty-printed JSON for an empty string
}

#[test]
fn test_to_vec_pretty_with_simple_integer() {
    let value = 42;
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"42"); // Pretty-printed JSON for an integer
}

#[test]
fn test_to_vec_pretty_with_simple_floating_point() {
    let value = 3.14;
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"3.14"); // Pretty-printed JSON for a float
}

#[test]
fn test_to_vec_pretty_with_boolean() {
    let value = true;
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"true"); // Pretty-printed JSON for a boolean
}

#[test]
fn test_to_vec_pretty_with_array() {
    let value = [1, 2, 3];
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"[1,2,3]"); // Pretty-printed JSON for an array
}

#[test]
fn test_to_vec_pretty_with_object() {
    let value = serde_json::json!({"key": "value", "number": 1});
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized, b"{\"key\":\"value\",\"number\":1}"); // Pretty-printed JSON for an object
}

#[test]
#[should_panic]
fn test_to_vec_pretty_with_non_serializable() {
    struct NonSerializable;
    
    let value = NonSerializable;
    let result = serde_json::to_vec_pretty(&value);
    assert!(result.is_err()); // Should trigger serialization error
}

