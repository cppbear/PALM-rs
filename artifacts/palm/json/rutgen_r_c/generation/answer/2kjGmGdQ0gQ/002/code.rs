// Answer 0

#[test]
fn test_to_string_with_valid_integer() {
    let value = &42; // Valid integer
    let result = serde_json::to_string(value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert_eq!(string, "42");
}

#[test]
fn test_to_string_with_valid_string() {
    let value = &"test"; // Valid string
    let result = serde_json::to_string(value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert_eq!(string, "\"test\"");
}

#[test]
fn test_to_string_with_valid_array() {
    let value = &["one", "two", "three"]; // Valid array
    let result = serde_json::to_string(value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert_eq!(string, "[\"one\",\"two\",\"three\"]");
}

#[test]
fn test_to_string_with_valid_object() {
    let value = &serde_json::json!({
        "name": "Alice",
        "age": 30,
        "is_student": false,
    }); // Valid object
    let result = serde_json::to_string(value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert_eq!(string, "{\"name\":\"Alice\",\"age\":30,\"is_student\":false}");
}

#[test]
fn test_to_string_with_empty_vec() {
    let value: &Vec<i32> = &vec![]; // Valid empty vector
    let result = serde_json::to_string(value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert_eq!(string, "[]");
}

#[test]
fn test_to_string_with_nested_array() {
    let value = &vec![vec![1, 2], vec![3, 4]]; // Valid nested array
    let result = serde_json::to_string(value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert_eq!(string, "[[1,2],[3,4]]");
}

