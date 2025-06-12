// Answer 0

#[test]
fn test_pointer_with_valid_object_path() {
    let data = Value::Object(Map {
        map: {
            let mut m = std::collections::BTreeMap::new();
            m.insert("x".to_string(), Value::Object(Map {
                map: {
                    let mut inner_m = std::collections::BTreeMap::new();
                    inner_m.insert("y".to_string(), Value::Array(vec![Value::String("z".to_string()), Value::String("zz".to_string())]));
                    inner_m
                }
            }));
            m
        }
    });

    assert_eq!(data.pointer("/x/y/1").unwrap(), &Value::String("zz".to_string()));
}

#[test]
fn test_pointer_with_invalid_object_path() {
    let data = Value::Object(Map {
        map: {
            let mut m = std::collections::BTreeMap::new();
            m.insert("x".to_string(), Value::Null);
            m
        }
    });

    assert_eq!(data.pointer("/a/b/c"), None);
}

#[test]
fn test_pointer_with_empty_string() {
    let data = Value::String("test".to_string());

    assert_eq!(data.pointer(""), Some(&data));
}

#[test]
fn test_pointer_without_leading_slash() {
    let data = Value::Object(Map {
        map: {
            let mut m = std::collections::BTreeMap::new();
            m.insert("x".to_string(), Value::Null);
            m
        }
    });

    assert_eq!(data.pointer("x"), None);
}

#[test]
fn test_pointer_with_array_index() {
    let data = Value::Array(vec![Value::String("first".to_string()), Value::String("second".to_string())]);

    assert_eq!(data.pointer("/1").unwrap(), &Value::String("second".to_string()));
}

#[test]
fn test_pointer_with_invalid_array_index() {
    let data = Value::Array(vec![Value::String("first".to_string()), Value::String("second".to_string())]);

    assert_eq!(data.pointer("/2"), None);
}

