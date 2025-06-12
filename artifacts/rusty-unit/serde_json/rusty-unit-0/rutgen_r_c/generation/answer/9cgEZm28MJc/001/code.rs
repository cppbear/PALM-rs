// Answer 0

#[test]
fn test_pointer_empty_string() {
    use serde_json::json;

    let data = json!({
        "key": "value"
    });

    assert_eq!(data.pointer(""), Some(&data));
}

#[test]
fn test_pointer_valid_path() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/x/y/1").unwrap(), &json!("zz"));
}

#[test]
fn test_pointer_invalid_path() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/a/b/c"), None);
}

#[test]
fn test_pointer_invalid_starting_character() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("x/y/1"), None);
}

#[test]
#[should_panic]
fn test_pointer_invalid_tokens() {
    use serde_json::json;

    let data = json!({
            "x": {
            "y": ["z", "zz"]
        }
    });

    // No panic expected since the function handles the invalid path safely.
    let _ = data.pointer("/x/y/1/2");
}

