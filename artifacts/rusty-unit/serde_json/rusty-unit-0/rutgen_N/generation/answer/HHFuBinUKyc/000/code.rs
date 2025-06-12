// Answer 0

#[test]
fn test_index_into_object() {
    // Define a simple JSON object
    let data = serde_json::json!({
        "key": "value",
        "array": [1, 2, 3]
    });

    // Accessing existing keys
    assert_eq!(data["key"], serde_json::json!("value"));
    assert_eq!(data["array"], serde_json::json!([1, 2, 3]));
}

#[test]
fn test_index_into_array() {
    // Define a simple JSON array
    let data = serde_json::json!([1, 2, 3, 4, 5]);

    // Accessing existing indices
    assert_eq!(data[0], serde_json::json!(1));
    assert_eq!(data[4], serde_json::json!(5));

    // Accessing an out of bounds index
    assert_eq!(data[5], serde_json::json!(null));
}

#[test]
fn test_index_into_nonexistent_key() {
    // Define a JSON object without certain keys
    let data = serde_json::json!({
        "key": "value"
    });

    // Accessing nonexistent keys
    assert_eq!(data["nonexistent"], serde_json::json!(null));
}

#[test]
fn test_index_null_return() {
    // Define a nested JSON object
    let data = serde_json::json!({
        "x": {
            "y": ["a", "b", "c"]
        }
    });

    // Accessing a key and index that do not exist
    assert_eq!(data["x"]["y"]["z"], serde_json::json!(null)); // string index on an array
    assert_eq!(data["a"]["b"], serde_json::json!(null)); // accessing nonexistent keys
    assert_eq!(data["x"][3], serde_json::json!(null)); // out of bounds index
}

