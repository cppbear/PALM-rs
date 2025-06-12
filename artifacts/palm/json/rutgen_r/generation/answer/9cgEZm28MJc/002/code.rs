// Answer 0

#[test]
fn test_pointer_valid_object_path() {
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
fn test_pointer_valid_array_index() {
    use serde_json::json;
    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/x/y/1").unwrap(), &json!("zz"));
}

#[test]
fn test_pointer_invalid_path_nonexistent() {
    use serde_json::json;
    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/a/b/c"), None);
}

#[test]
fn test_pointer_empty_path() {
    use serde_json::json;
    let data = json!({
        "k": "value"
    });

    assert_eq!(data.pointer(""), Some(&data));
}

#[test]
fn test_pointer_invalid_starting_character() {
    use serde_json::json;
    let data = json!({
        "a": "value"
    });

    assert_eq!(data.pointer("a/b"), None);
}

#[test]
fn test_pointer_with_escape_sequences() {
    use serde_json::json;
    let data = json!({
        "a~1b": "value"
    });

    assert_eq!(data.pointer("/a~1b").unwrap(), &json!("value"));
}

#[test]
fn test_pointer_with_nested_objects() {
    use serde_json::json;
    let data = json!({
        "outer": {
            "inner": {
                "key": "nested value"
            }
        }
    });

    assert_eq!(data.pointer("/outer/inner/key").unwrap(), &json!("nested value"));
}

#[test]
fn test_pointer_array_out_of_bounds() {
    use serde_json::json;
    let data = json!({
        "arr": [1, 2, 3]
    });

    assert_eq!(data.pointer("/arr/5"), None);
}

