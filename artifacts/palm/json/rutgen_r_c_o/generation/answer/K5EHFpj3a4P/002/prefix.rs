// Answer 0

#[test]
fn test_as_bool_true() {
    let value = Value::Bool(true);
    let result = value.as_bool();
}

#[test]
fn test_as_bool_false() {
    let value = Value::Bool(false);
    let result = value.as_bool();
}

#[test]
fn test_as_bool_null() {
    let value = Value::Null;
    let result = value.as_bool();
}

#[test]
fn test_as_bool_number() {
    let value = Value::Number(Number { n: 1 });
    let result = value.as_bool();
}

#[test]
fn test_as_bool_string() {
    let value = Value::String(String::from("false"));
    let result = value.as_bool();
}

#[test]
fn test_as_bool_array() {
    let value = Value::Array(vec![Value::Bool(true)]);
    let result = value.as_bool();
}

#[test]
fn test_as_bool_object() {
    let mut object_map = Map::new();
    object_map.insert(String::from("key"), Value::Bool(false));
    let value = Value::Object(object_map);
    let result = value.as_bool();
}

