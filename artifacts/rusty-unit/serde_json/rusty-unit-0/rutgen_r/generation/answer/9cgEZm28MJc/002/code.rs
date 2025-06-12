// Answer 0

#[test]
fn test_pointer_valid_object_path() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/x/y/1").unwrap(), &json!("zz"));
}

#[test]
fn test_pointer_valid_empty_path() {
    use serde_json::json;

    let data = json!({
        "key": "value"
    });

    assert_eq!(data.pointer("").unwrap(), &data);
}

#[test]
fn test_pointer_invalid_start() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("x/y/1"), None);
}

#[test]
fn test_pointer_non_existent_path() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/a/b/c"), None);
}

#[test]
fn test_pointer_valid_array_index() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/x/y/0").unwrap(), &json!("z"));
}

#[test]
fn test_pointer_invalid_array_index() {
    use serde_json::json;

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/x/y/2"), None);
}

#[test]
fn test_pointer_escaped_characters() {
    use serde_json::json;

    let data = json!({
        "foo~bar": {
            "baz": "qux"
        }
    });

    assert_eq!(data.pointer("/foo~0bar/baz").unwrap(), &json!("qux"));
}

#[test]
fn test_pointer_valid_nested_structure() {
    use serde_json::json;

    let data = json!({
        "a": {
            "b": {
                "c": "d"
            }
        }
    });

    assert_eq!(data.pointer("/a/b/c").unwrap(), &json!("d"));
}

