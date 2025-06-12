// Answer 0

#[test]
fn test_index_with_valid_string_key() {
    let value = serde_json::json!({
        "key": "value"
    });
    assert_eq!(value["key"], serde_json::json!("value"));
}

#[test]
fn test_index_with_missing_string_key() {
    let value = serde_json::json!({
        "key": "value"
    });
    assert_eq!(value["missing_key"], serde_json::json!(null));
}

#[test]
fn test_index_with_valid_numeric_index() {
    let value = serde_json::json!(["a", "b", "c"]);
    assert_eq!(value[1], serde_json::json!("b"));
}

#[test]
fn test_index_with_out_of_bounds_numeric_index() {
    let value = serde_json::json!(["a", "b", "c"]);
    assert_eq!(value[5], serde_json::json!(null));
}

#[test]
fn test_index_into_map_with_invalid_type() {
    let value = serde_json::json!(["a", "b", "c"]);
    assert_eq!(value["key"], serde_json::json!(null));
}

#[test]
fn test_index_into_nested_structure() {
    let value = serde_json::json!({
        "outer": {
            "inner": ["x", "y"]
        }
    });
    assert_eq!(value["outer"]["inner"][0], serde_json::json!("x"));
    assert_eq!(value["outer"]["inner"][1], serde_json::json!("y"));
}

#[test]
fn test_index_into_deeply_nested_missing_key() {
    let value = serde_json::json!({
        "outer": {
            "inner": []
        }
    });
    assert_eq!(value["outer"]["inner"]["missing_key"], serde_json::json!(null));
}

#[test]
#[should_panic]
fn test_index_into_map_panic_with_invalid_path() {
    let value = serde_json::json!({
        "key": {
            "subkey": "value"
        }
    });
    // Attempting to index a non-existing key in an array-like manner should panic.
    let _ = value["key"][1];
}

