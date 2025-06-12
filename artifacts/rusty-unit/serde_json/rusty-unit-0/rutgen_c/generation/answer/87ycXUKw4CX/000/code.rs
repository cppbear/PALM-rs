// Answer 0

#[test]
fn test_into_deserializer_with_null() {
    let value = Value::Null;
    let deserializer = value.into_deserializer();
    assert!(matches!(deserializer, Value::Null));
}

#[test]
fn test_into_deserializer_with_bool() {
    let value = Value::Bool(true);
    let deserializer = value.into_deserializer();
    assert!(matches!(deserializer, Value::Bool(true)));
}

#[test]
fn test_into_deserializer_with_number() {
    let number = Number { n: 42 }; // Assuming N = i32
    let value = Value::Number(number);
    let deserializer = value.into_deserializer();
    assert!(matches!(deserializer, Value::Number(_)));
}

#[test]
fn test_into_deserializer_with_string() {
    let value = Value::String(String::from("test"));
    let deserializer = value.into_deserializer();
    assert!(matches!(deserializer, Value::String(ref s) if s == "test"));
}

#[test]
fn test_into_deserializer_with_array() {
    let value = Value::Array(vec![Value::Null, Value::Bool(false)]);
    let deserializer = value.into_deserializer();
    assert!(matches!(deserializer, Value::Array(ref arr) if arr.len() == 2));
}

#[test]
fn test_into_deserializer_with_object() {
    let mut map = Map { map: MapImpl::new() }; // Assuming MapImpl::new() exists
    map.map.insert(String::from("key"), Value::Bool(true)); // Insert method assumed
    let value = Value::Object(map);
    let deserializer = value.into_deserializer();
    assert!(matches!(deserializer, Value::Object(_)));
}

