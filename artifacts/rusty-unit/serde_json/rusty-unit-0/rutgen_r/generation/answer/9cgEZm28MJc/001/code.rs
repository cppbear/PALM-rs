// Answer 0

#[test]
fn test_pointer_empty_string() {
    use serde_json::json;
    use serde_json::Value;

    let data = json!({
        "key": "value"
    });

    assert_eq!(data.pointer(""), Some(&data));
}

#[test]
fn test_pointer_single_root_slash() {
    use serde_json::json;
    use serde_json::Value;

    let data = json!({
        "key": "value"
    });

    assert_eq!(data.pointer("/"), Some(&data));
}

#[test]
fn test_pointer_invalid_start() {
    use serde_json::json;
    use serde_json::Value;

    let data = json!({
        "key": "value"
    });

    assert_eq!(data.pointer("key"), None);
}

#[test]
fn test_pointer_nonexistent_field() {
    use serde_json::json;
    use serde_json::Value;

    let data = json!({
        "key": "value"
    });

    assert_eq!(data.pointer("/nonexistent"), None);
}

#[test]
fn test_pointer_nested_value() {
    use serde_json::json;
    use serde_json::Value;

    let data = json!({
        "outer": {
            "inner": {
                "value": "test"
            }
        }
    });

    assert_eq!(data.pointer("/outer/inner/value"), Some(&json!("test")));
}

