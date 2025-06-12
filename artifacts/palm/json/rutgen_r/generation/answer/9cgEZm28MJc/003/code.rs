// Answer 0

#[test]
fn test_pointer_invalid_start_without_slash() {
    let data = serde_json::json!({
        "a": 1,
        "b": 2,
        "c": {
            "d": 3
        }
    });

    assert_eq!(data.pointer("x/y/z"), None);
}

#[test]
fn test_pointer_empty_string() {
    let data = serde_json::json!({
        "key": "value"
    });

    assert_eq!(data.pointer(""), Some(&data));
}

