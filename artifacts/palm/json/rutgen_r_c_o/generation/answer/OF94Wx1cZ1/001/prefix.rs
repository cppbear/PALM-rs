// Answer 0

#[test]
fn test_is_boolean_true() {
    let value = Value::Bool(true);
    value.is_boolean();
}

#[test]
fn test_is_boolean_false() {
    let value = Value::Bool(false);
    value.is_boolean();
}

#[test]
fn test_is_boolean_number() {
    let value = Value::Number(Number { n: 0 });
    value.is_boolean();
}

#[test]
fn test_is_boolean_string() {
    let value = Value::String(String::from("false"));
    value.is_boolean();
}

#[test]
fn test_is_boolean_array() {
    let value = Value::Array(Vec::new());
    value.is_boolean();
}

#[test]
fn test_is_boolean_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    value.is_boolean();
}

#[test]
fn test_is_boolean_null() {
    let value = Value::Null;
    value.is_boolean();
}

