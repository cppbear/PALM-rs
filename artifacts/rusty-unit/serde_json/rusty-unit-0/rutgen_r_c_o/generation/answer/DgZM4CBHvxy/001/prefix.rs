// Answer 0

#[test]
fn test_take_null() {
    let mut v = Value::Null;
    v.take();
}

#[test]
fn test_take_bool_true() {
    let mut v = Value::Bool(true);
    v.take();
}

#[test]
fn test_take_bool_false() {
    let mut v = Value::Bool(false);
    v.take();
}

#[test]
fn test_take_number_zero() {
    let mut v = Value::Number(Number::from(0));
    v.take();
}

#[test]
fn test_take_number_negative_one() {
    let mut v = Value::Number(Number::from(-1));
    v.take();
}

#[test]
fn test_take_number_one() {
    let mut v = Value::Number(Number::from(1));
    v.take();
}

#[test]
fn test_take_number_max() {
    let mut v = Value::Number(Number::from(f64::MAX));
    v.take();
}

#[test]
fn test_take_empty_string() {
    let mut v = Value::String(String::from(""));
    v.take();
}

#[test]
fn test_take_non_empty_string() {
    let mut v = Value::String(String::from("test"));
    v.take();
}

#[test]
fn test_take_empty_array() {
    let mut v = Value::Array(vec![]);
    v.take();
}

#[test]
fn test_take_array_with_null() {
    let mut v = Value::Array(vec![Value::Null]);
    v.take();
}

#[test]
fn test_take_array_with_bool() {
    let mut v = Value::Array(vec![Value::Bool(true)]);
    v.take();
}

#[test]
fn test_take_empty_object() {
    let mut v = Value::Object(Map::new());
    v.take();
}

#[test]
fn test_take_object_with_key_value() {
    let mut v = Value::Object(Map::from_iter(vec![(String::from("key"), Value::String(String::from("value")))]));
    v.take();
}

