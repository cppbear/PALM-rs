// Answer 0

#[test]
fn test_eq_bool_true_with_bool_true() {
    let value = Value::Bool(true);
    let other = true;
    eq_bool(&value, other);
}

#[test]
fn test_eq_bool_true_with_bool_false() {
    let value = Value::Bool(true);
    let other = false;
    eq_bool(&value, other);
}

#[test]
fn test_eq_bool_false_with_bool_true() {
    let value = Value::Bool(false);
    let other = true;
    eq_bool(&value, other);
}

#[test]
fn test_eq_bool_false_with_bool_false() {
    let value = Value::Bool(false);
    let other = false;
    eq_bool(&value, other);
}

#[test]
fn test_eq_bool_with_null() {
    let value = Value::Null;
    let other = true;
    eq_bool(&value, other);
}

#[test]
fn test_eq_bool_with_number() {
    let value = Value::Number(Number::from(0));
    let other = true;
    eq_bool(&value, other);
}

#[test]
fn test_eq_bool_with_string() {
    let value = Value::String(String::from("test"));
    let other = true;
    eq_bool(&value, other);
}

#[test]
fn test_eq_bool_with_array() {
    let value = Value::Array(vec![Value::Bool(true)]);
    let other = true;
    eq_bool(&value, other);
}

#[test]
fn test_eq_bool_with_object() {
    let value = Value::Object(Map::new());
    let other = true;
    eq_bool(&value, other);
}

