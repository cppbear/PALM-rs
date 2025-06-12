// Answer 0

#[test]
fn test_index_existing_key() {
    let data = serde_json::json!({
        "key1": "value1",
        "key2": "value2"
    });

    assert_eq!(data["key1"], serde_json::json!("value1"));
}

#[test]
fn test_index_non_existing_key() {
    let data = serde_json::json!({
        "key1": "value1",
        "key2": "value2"
    });

    assert_eq!(data["non_existing_key"], serde_json::json!(null));
}

#[test]
fn test_index_into_array_within_bounds() {
    let data = serde_json::json!([
        "first",
        "second",
        "third"
    ]);

    assert_eq!(data[1], serde_json::json!("second"));
}

#[test]
fn test_index_into_array_out_of_bounds() {
    let data = serde_json::json!([
        "first",
        "second",
        "third"
    ]);

    assert_eq!(data[3], serde_json::json!(null));
}

#[test]
fn test_index_nested_json() {
    let data = serde_json::json!({
        "outer": {
            "inner": ["deep_value"]
        }
    });

    assert_eq!(data["outer"]["inner"][0], serde_json::json!("deep_value"));
}

#[test]
fn test_index_nested_json_with_non_existing_key() {
    let data = serde_json::json!({
        "outer": {
            "inner": ["deep_value"]
        }
    });

    assert_eq!(data["outer"]["non_existing_key"], serde_json::json!(null));
}

