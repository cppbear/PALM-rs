// Answer 0

#[test]
fn test_into_deserializer_null() {
    let value = Value::Null;
    let deserializer = value.into_deserializer();
}

#[test]
fn test_into_deserializer_bool() {
    let value = Value::Bool(true);
    let deserializer = value.into_deserializer();
}

#[test]
fn test_into_deserializer_number() {
    let value = Value::Number(Number { n: 0 });
    let deserializer = value.into_deserializer();
}

#[test]
fn test_into_deserializer_empty_string() {
    let value = Value::String(String::from(""));
    let deserializer = value.into_deserializer();
}

#[test]
fn test_into_deserializer_array() {
    let value = Value::Array(vec![Value::Null]);
    let deserializer = value.into_deserializer();
}

#[test]
fn test_into_deserializer_object() {
    let value = Value::Object(Map::new());
    let deserializer = value.into_deserializer();
}

