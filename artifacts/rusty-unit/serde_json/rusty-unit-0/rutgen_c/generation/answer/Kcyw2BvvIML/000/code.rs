// Answer 0

#[test]
fn test_to_value_null() {
    let v: Value = to_value(()).unwrap();
    assert_eq!(v, Value::Null);
}

#[test]
fn test_to_value_bool() {
    let v: Value = to_value(true).unwrap();
    assert_eq!(v, Value::Bool(true));
}

#[test]
fn test_to_value_number() {
    let v: Value = to_value(12.5).unwrap();
    assert_eq!(v, Value::Number(Number::from_f64(12.5).unwrap()));
}

#[test]
fn test_to_value_string() {
    let v: Value = to_value("a string").unwrap();
    assert_eq!(v, Value::String(String::from("a string")));
}

#[test]
fn test_to_value_array() {
    let v: Value = to_value(vec![Value::Null, Value::Bool(false)]).unwrap();
    assert_eq!(v, Value::Array(vec![Value::Null, Value::Bool(false)]));
}

#[test]
fn test_to_value_object() {
    let mut obj = std::collections::BTreeMap::new();
    obj.insert("key1".to_string(), Value::String("value1".to_string()));
    obj.insert("key2".to_string(), Value::Number(Number::from_f64(42.0).unwrap()));
    let v: Value = to_value(obj).unwrap();
    let expected = Value::Object(Map::from_iter([
        (String::from("key1"), Value::String(String::from("value1"))),
        (String::from("key2"), Value::Number(Number::from_f64(42.0).unwrap())),
    ]));
    assert_eq!(v, expected);
}

#[test]
#[should_panic]
fn test_to_value_non_string_key() {
    let mut map = std::collections::BTreeMap::new();
    map.insert(vec![32, 64], "x86");
    let _ = to_value(map).unwrap();
}

