// Answer 0

#[test]
fn test_pointer_mut_with_empty_pointer() {
    let mut value = Value::Null;
    assert_eq!(value.pointer_mut(""), Some(&mut value));
}

#[test]
fn test_pointer_mut_with_invalid_pointer() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    assert_eq!(value.pointer_mut("invalid_pointer"), None);
}

#[test]
fn test_pointer_mut_with_valid_object_pointer() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    let inner_key = "key";
    value.as_object_mut().unwrap().map.insert(inner_key.to_string(), Value::Number(Number { n: 42 }));

    assert_eq!(value.pointer_mut("/key"), Some(&mut Value::Number(Number { n: 42 })));
}

#[test]
fn test_pointer_mut_with_nonexistent_object_pointer() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    assert_eq!(value.pointer_mut("/non_existent"), None);
}

#[test]
fn test_pointer_mut_with_valid_array_pointer() {
    let mut value = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    assert_eq!(value.pointer_mut("/1"), Some(&mut Value::Number(Number { n: 2 })));
}

#[test]
fn test_pointer_mut_with_nonexistent_array_pointer() {
    let mut value = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    assert_eq!(value.pointer_mut("/5"), None);
}

#[test]
fn test_pointer_mut_with_nested_object_pointer() {
    let mut nested_value = Value::Object(Map { map: MapImpl::new() });
    nested_value.as_object_mut().unwrap().map.insert("inner_key".to_string(), Value::Number(Number { n: 100 }));

    let mut value = Value::Object(Map { map: MapImpl::new() });
    value.as_object_mut().unwrap().map.insert("outer_key".to_string(), nested_value);

    assert_eq!(value.pointer_mut("/outer_key/inner_key"), Some(&mut Value::Number(Number { n: 100 })));
}

#[test]
fn test_pointer_mut_with_invalid_nested_pointer() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    value.as_object_mut().unwrap().map.insert("key".to_string(), Value::Number(Number { n: 5 }));

    assert_eq!(value.pointer_mut("/key/invalid"), None);
}

#[test]
fn test_pointer_mut_with_string_value() {
    let mut value = Value::String("Hello".to_string());
    assert_eq!(value.pointer_mut("/"), Some(&mut value));
}

