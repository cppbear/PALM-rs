// Answer 0

#[test]
fn test_pointer_empty_string() {
    use serde_json::{Value, json};

    let data = json!({
        "key": "value"
    });

    assert_eq!(data.pointer(""), Some(&data));
}

#[test]
fn test_pointer_valid_key() {
    use serde_json::{Value, json};

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/x/y/1").unwrap(), &json!("zz"));
}

#[test]
fn test_pointer_invalid_key() {
    use serde_json::{Value, json};

    let data = json!({
        "x": {
            "y": ["z", "zz"]
        }
    });

    assert_eq!(data.pointer("/a/b/c"), None);
}

#[test]
fn test_pointer_encoded_chars() {
    use serde_json::{Value, json};

    let data = json!({
        "a~b": {
            "c": "value"
        }
    });

    assert_eq!(data.pointer("/a~0b/c").unwrap(), &json!("value"));
}

#[test]
fn test_pointer_array_index() {
    use serde_json::{Value, json};

    let data = json!({
        "array": ["first", "second", "third"]
    });

    assert_eq!(data.pointer("/array/1").unwrap(), &json!("second"));
}

#[test]
fn test_pointer_array_out_of_bounds() {
    use serde_json::{Value, json};

    let data = json!({
        "array": ["first", "second"]
    });

    assert_eq!(data.pointer("/array/2"), None);
}

