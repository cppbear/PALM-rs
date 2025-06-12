// Answer 0

#[test]
fn test_as_array_mut_with_array() {
    let mut value = Value::Array(vec![Value::String("item1".to_string()), Value::String("item2".to_string())]);
    let array_mut = value.as_array_mut().unwrap();
    array_mut.clear();
    assert_eq!(value, Value::Array(vec![]));
}

#[test]
fn test_as_array_mut_with_object() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    let array_mut = value.as_array_mut();
    assert!(array_mut.is_none());
}

#[test]
fn test_as_array_mut_with_null() {
    let mut value = Value::Null;
    let array_mut = value.as_array_mut();
    assert!(array_mut.is_none());
}

#[test]
fn test_as_array_mut_with_number() {
    let mut value = Value::Number(Number { n: 42 });
    let array_mut = value.as_array_mut();
    assert!(array_mut.is_none());
}

#[test]
fn test_as_array_mut_empty() {
    let mut value = Value::Array(vec![]);
    let array_mut = value.as_array_mut().unwrap();
    assert!(array_mut.is_empty());
}

