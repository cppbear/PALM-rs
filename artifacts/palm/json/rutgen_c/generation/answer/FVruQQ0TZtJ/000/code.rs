// Answer 0

#[test]
fn test_is_f64_with_float_value() {
    let value = Value::Number(Number::from_f64(256.0).unwrap());
    assert!(value.is_f64());
}

#[test]
fn test_is_f64_with_integer_value() {
    let positive_integer = Value::Number(Number::from_i128(64).unwrap());
    let negative_integer = Value::Number(Number::from_i128(-64).unwrap());
    
    assert!(!positive_integer.is_f64());
    assert!(!negative_integer.is_f64());
}

#[test]
fn test_is_f64_with_null_value() {
    let null_value = Value::Null;
    assert!(!null_value.is_f64());
}

#[test]
fn test_is_f64_with_boolean_value() {
    let true_value = Value::Bool(true);
    let false_value = Value::Bool(false);
    
    assert!(!true_value.is_f64());
    assert!(!false_value.is_f64());
}

#[test]
fn test_is_f64_with_string_value() {
    let string_value = Value::String(String::from("example"));
    assert!(!string_value.is_f64());
}

#[test]
fn test_is_f64_with_array_value() {
    let array_value = Value::Array(vec![Value::Number(Number::from_f64(3.14).unwrap())]);
    assert!(!array_value.is_f64());
}

#[test]
fn test_is_f64_with_object_value() {
    let object_value = Value::Object(Map { map: MapImpl::new() });
    assert!(!object_value.is_f64());
}

