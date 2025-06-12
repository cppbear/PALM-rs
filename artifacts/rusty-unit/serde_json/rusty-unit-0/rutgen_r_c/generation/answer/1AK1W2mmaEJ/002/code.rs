// Answer 0

#[test]
fn test_as_array_mut_with_empty_array() {
    let mut value = Value::Array(Vec::new());
    let result = value.as_array_mut();
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_as_array_mut_with_non_empty_array() {
    let mut value = Value::Array(vec![Value::String("test".to_string()), Value::Number(Number { n: 42 })]);
    let result = value.as_array_mut();
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_as_array_mut_with_object() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    let result = value.as_array_mut();
    assert!(result.is_none());
}

#[test]
fn test_as_array_mut_with_null() {
    let mut value = Value::Null;
    let result = value.as_array_mut();
    assert!(result.is_none());
}

#[test]
fn test_as_array_mut_with_boolean() {
    let mut value = Value::Bool(true);
    let result = value.as_array_mut();
    assert!(result.is_none());
}

