// Answer 0

#[test]
fn test_value_as_null_when_value_is_null() {
    use serde_json::Value;

    let value = Value::Null;
    assert_eq!(value.as_null(), Some(()));
}

#[test]
fn test_value_as_null_when_value_is_not_null() {
    use serde_json::Value;

    let value = Value::Bool(true);
    assert_eq!(value.as_null(), None);

    let value_string = Value::String(String::from("not null"));
    assert_eq!(value_string.as_null(), None);
    
    let value_number = Value::Number(Number { n: 0 });
    assert_eq!(value_number.as_null(), None);

    let value_array = Value::Array(Vec::new());
    assert_eq!(value_array.as_null(), None);

    let value_object = Value::Object(Map::new());
    assert_eq!(value_object.as_null(), None);
}

