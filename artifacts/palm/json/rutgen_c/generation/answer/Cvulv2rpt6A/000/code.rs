// Answer 0

#[test]
fn test_eq_f32_with_number_matching() {
    let number = Number { n: N::Float(3.14) };
    let value = Value::Number(number);
    assert!(eq_f32(&value, 3.14));
}

#[test]
fn test_eq_f32_with_number_not_matching() {
    let number = Number { n: N::Float(3.14) };
    let value = Value::Number(number);
    assert!(!eq_f32(&value, 2.71));
}

#[test]
fn test_eq_f32_with_null() {
    let value = Value::Null;
    assert!(!eq_f32(&value, 3.14));
}

#[test]
fn test_eq_f32_with_bool() {
    let value = Value::Bool(true);
    assert!(!eq_f32(&value, 3.14));
}

#[test]
fn test_eq_f32_with_string() {
    let value = Value::String(String::from("test"));
    assert!(!eq_f32(&value, 3.14));
}

#[test]
fn test_eq_f32_with_array() {
    let value = Value::Array(vec![]);
    assert!(!eq_f32(&value, 3.14));
}

#[test]
fn test_eq_f32_with_object() {
    let value = Value::Object(Map::new());
    assert!(!eq_f32(&value, 3.14));
}

