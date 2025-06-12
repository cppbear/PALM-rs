// Answer 0

#[test]
fn test_pointer_empty_string() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    // Testing with an empty pointer (should return Some(self))
    assert_eq!(data.pointer(""), Some(&data));
}

#[test]
fn test_pointer_invalid_start() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    // Testing with a pointer that does not start with '/' (should return None)
    assert_eq!(data.pointer("x/y/1"), None);
}

#[test]
fn test_pointer_invalid_start_with_tilde() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    // Testing with a pointer that does not start with '/' including escape character (should return None)
    assert_eq!(data.pointer("~0"), None);
}

