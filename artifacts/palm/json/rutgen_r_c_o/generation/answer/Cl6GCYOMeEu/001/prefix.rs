// Answer 0

#[test]
fn test_eq_i64_with_null() {
    let value = Value::Null;
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
}

#[test]
fn test_eq_i64_with_bool_true() {
    let value = Value::Bool(true);
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
}

#[test]
fn test_eq_i64_with_bool_false() {
    let value = Value::Bool(false);
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
}

#[test]
fn test_eq_i64_with_number_negative_128() {
    let value = Value::Number(Number::from_i64(-128));
    let _ = eq_i64(&value, -128);
    let _ = eq_i64(&value, -127);
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
    let _ = eq_i64(&value, 127);
    let _ = eq_i64(&value, 128);
}

#[test]
fn test_eq_i64_with_number_positive_127() {
    let value = Value::Number(Number::from_i64(127));
    let _ = eq_i64(&value, -128);
    let _ = eq_i64(&value, -127);
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
    let _ = eq_i64(&value, 127);
    let _ = eq_i64(&value, 128);
}

#[test]
fn test_eq_i64_with_number_zero() {
    let value = Value::Number(Number::from_i64(0));
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
}

#[test]
fn test_eq_i64_with_number_positive_128() {
    let value = Value::Number(Number::from_i64(128));
    let _ = eq_i64(&value, -128);
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
    let _ = eq_i64(&value, 127);
    let _ = eq_i64(&value, 128);
}

#[test]
fn test_eq_i64_with_string() {
    let value = Value::String(String::from("test"));
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
}

#[test]
fn test_eq_i64_with_array() {
    let value = Value::Array(vec![Value::Number(Number::from_i64(0))]);
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
}

#[test]
fn test_eq_i64_with_object() {
    let value = Value::Object(Map::new());
    let _ = eq_i64(&value, -1);
    let _ = eq_i64(&value, 0);
    let _ = eq_i64(&value, 1);
}

