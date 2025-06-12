// Answer 0

#[test]
fn test_is_object_empty_object() {
    let obj = Value::Object(Map::<String, Value>::new());
    obj.is_object();
}

#[test]
fn test_is_object_non_empty_object() {
    let mut map = Map::<String, Value>::new();
    map.map.insert(String::from("key"), Value::Number(Number { n: N::default() }));
    let obj = Value::Object(map);
    obj.is_object();
}

#[test]
fn test_is_object_array() {
    let array = Value::Array(Vec::new());
    array.is_object();
}

#[test]
fn test_is_object_null() {
    let null_value = Value::Null;
    null_value.is_object();
}

#[test]
fn test_is_object_boolean() {
    let boolean_value = Value::Bool(false);
    boolean_value.is_object();
}

#[test]
fn test_is_object_empty_string() {
    let string_value = Value::String(String::from(""));
    string_value.is_object();
}

#[test]
fn test_is_object_number() {
    let number_value = Value::Number(Number { n: N::default() });
    number_value.is_object();
}

