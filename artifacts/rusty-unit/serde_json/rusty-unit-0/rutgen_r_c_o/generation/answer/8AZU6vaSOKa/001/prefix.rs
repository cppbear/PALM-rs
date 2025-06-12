// Answer 0

#[test]
fn test_is_i64_with_bool() {
    let value = Value::Bool(true);
    value.is_i64();
}

#[test]
fn test_is_i64_with_null() {
    let value = Value::Null;
    value.is_i64();
}

#[test]
fn test_is_i64_with_string() {
    let value = Value::String(String::from("not a number"));
    value.is_i64();
}

#[test]
fn test_is_i64_with_array() {
    let value = Value::Array(vec![
        Value::Number(Number::from_i64(42).unwrap()), 
        Value::String(String::from("example"))
    ]);
    value.is_i64();
}

#[test]
fn test_is_i64_with_object() {
    let mut object_map = Map {
        map: MapImpl::new(),
    };
    object_map.map.insert(String::from("key1"), Value::String(String::from("value1")));
    let value = Value::Object(object_map);
    value.is_i64();
}

