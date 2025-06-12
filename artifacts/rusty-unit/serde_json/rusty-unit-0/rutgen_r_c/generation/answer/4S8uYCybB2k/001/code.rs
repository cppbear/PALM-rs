// Answer 0

#[test]
fn test_value_as_u64_none_non_number() {
    let value_null = Value::Null;
    let value_bool = Value::Bool(true);
    let value_string = Value::String(String::from("test"));
    let value_array = Value::Array(vec![Value::Bool(false)]);
    let value_object = Value::Object(Map::new());

    assert_eq!(value_null.as_u64(), None);
    assert_eq!(value_bool.as_u64(), None);
    assert_eq!(value_string.as_u64(), None);
    assert_eq!(value_array.as_u64(), None);
    assert_eq!(value_object.as_u64(), None);
}

#[test]
fn test_value_as_u64_none_float() {
    let value_float = Value::Number(Number::from_f64(12.34).unwrap());
    assert_eq!(value_float.as_u64(), None);
}

