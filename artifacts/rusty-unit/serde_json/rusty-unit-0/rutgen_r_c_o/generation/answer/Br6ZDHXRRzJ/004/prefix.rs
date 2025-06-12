// Answer 0

#[test]
fn test_unexpected_null() {
    let value = Value::Null;
    value.unexpected();
}

#[test]
fn test_unexpected_bool_true() {
    let value = Value::Bool(true);
    value.unexpected();
}

#[test]
fn test_unexpected_bool_false() {
    let value = Value::Bool(false);
    value.unexpected();
}

#[test]
fn test_unexpected_number_float() {
    let number = Number { n: 12.5 };
    let value = Value::Number(number);
    value.unexpected();
}

#[test]
fn test_unexpected_number_negative_float() {
    let number = Number { n: -12.5 };
    let value = Value::Number(number);
    value.unexpected();
}

#[test]
fn test_unexpected_number_integer() {
    let number = Number { n: 42 };
    let value = Value::Number(number);
    value.unexpected();
}

#[test]
fn test_unexpected_string() {
    let value = Value::String(String::from("a string"));
    value.unexpected();
}

#[test]
fn test_unexpected_array() {
    let value = Value::Array(vec![Value::Null, Value::Bool(false)]);
    value.unexpected();
}

#[test]
fn test_unexpected_object() {
    let mut map = Map::new();
    map.insert(String::from("key"), Value::Bool(true));
    let value = Value::Object(map);
    value.unexpected();
}

