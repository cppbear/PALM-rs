// Answer 0

#[test]
fn test_index_mut_with_existing_key() {
    let mut data = serde_json::json!({ "x": 0 });
    data["x"] = serde_json::json!(1);
    assert_eq!(data["x"], 1);
}

#[test]
fn test_index_mut_with_insert_new_key() {
    let mut data = serde_json::json!({ "x": 0 });
    data["y"] = serde_json::json!([false, false, false]);
    assert_eq!(data["y"], serde_json::json!([false, false, false]));
}

#[test]
fn test_index_mut_with_replace_array_value() {
    let mut data = serde_json::json!({ "y": [false, false, false] });
    data["y"][0] = serde_json::json!(true);
    assert_eq!(data["y"][0], true);
}

#[test]
fn test_index_mut_with_deeply_nested_key() {
    let mut data = serde_json::json!({});
    data["a"]["b"]["c"]["d"] = serde_json::json!(true);
    assert_eq!(data["a"]["b"]["c"]["d"], true);
}

#[test]
#[should_panic]
fn test_index_mut_with_invalid_index_on_non_array() {
    let mut data = serde_json::json!(10);
    let _ = data[0]; // This should panic as it's not an array.
}

#[test]
#[should_panic]
fn test_index_mut_with_too_large_index() {
    let mut data = serde_json::json!([1, 2]);
    let _ = data[3]; // This should panic as the index is out of bounds.
}

#[test]
#[should_panic]
fn test_index_mut_with_non_object_index() {
    let mut data = serde_json::json!(10);
    let _ = data["key"]; // This should panic as it's not an object.
}

#[test]
fn test_index_mut_with_null_index() {
    let mut data = serde_json::json!(null);
    data["new_key"] = serde_json::json!(true);
    assert_eq!(data["new_key"], true);
}

