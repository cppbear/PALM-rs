// Answer 0

#[test]
fn test_serialize_some_bool() {
    let serializer = Serializer;
    let value = &true;
    let result = serializer.serialize_some(value).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_serialize_some_i32() {
    let serializer = Serializer;
    let value = &42i32;
    let result = serializer.serialize_some(value).unwrap();
    assert_eq!(result, Value::Number(Number::from(42)));
}

#[test]
fn test_serialize_some_string() {
    let serializer = Serializer;
    let value = &"Hello, world!".to_string();
    let result = serializer.serialize_some(value).unwrap();
    assert_eq!(result, Value::String("Hello, world!".to_string()));
}

#[test]
fn test_serialize_some_float() {
    let serializer = Serializer;
    let value = &3.14f64;
    let result = serializer.serialize_some(value).unwrap();
    assert_eq!(result, Value::Number(Number::from(3.14)));
}

#[test]
fn test_serialize_some_none() {
    let serializer = Serializer;
    let value: Option<i32> = None;
    let result = serializer.serialize_some(&value);
    assert!(result.is_err());
}

#[test]
fn test_serialize_some_empty_string() {
    let serializer = Serializer;
    let value = &"".to_string();
    let result = serializer.serialize_some(value).unwrap();
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_serialize_some_array() {
    let serializer = Serializer;
    let value = &vec![1, 2, 3];
    let result = serializer.serialize_some(value).unwrap();
    assert_eq!(result, Value::Array(vec![
        Value::Number(Number::from(1)),
        Value::Number(Number::from(2)),
        Value::Number(Number::from(3)),
    ]));
}

