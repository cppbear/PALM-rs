// Answer 0

#[test]
fn test_pointer_empty() {
    use serde_json::json;
    use serde_json::Value;

    let data = json!({
        "key1": "value1",
        "key2": {
            "nested_key": "nested_value"
        },
        "key3": [1, 2, 3]
    });

    assert_eq!(data.pointer(""), Some(&data));
}

