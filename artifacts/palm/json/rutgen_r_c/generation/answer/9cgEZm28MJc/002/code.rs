// Answer 0

#[test]
fn test_pointer_valid_object() {
    use serde_json::json;

    let data = json!({
        "a": {
            "b": {
                "c": "value"
            }
        }
    });

    assert_eq!(data.pointer("/a/b/c").unwrap(), &json!("value"));
}

#[test]
fn test_pointer_valid_array() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["first", "second"]
        }
    });

    assert_eq!(data.pointer("/x/y/1").unwrap(), &json!("second"));
}

#[test]
fn test_pointer_invalid_start() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["a", "b"]
        }
    });

    assert_eq!(data.pointer("x/y/0"), None); // Should return None because it does not start with '/'
}

#[test]
fn test_pointer_empty() {
    use serde_json::json;

    let data = json!({
        "key": "value"
    });

    assert_eq!(data.pointer(""), Some(&data)); // Should return Some(self) for empty string
}

#[test]
fn test_pointer_not_found() {
    use serde_json::json;

    let data = json!({
        "obj": {
            "nested": {}
        }
    });

    assert_eq!(data.pointer("/obj/nested/invalid"), None); // Should return None for a non-existing path
}

