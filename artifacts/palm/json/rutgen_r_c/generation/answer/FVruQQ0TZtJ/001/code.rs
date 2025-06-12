// Answer 0

#[test]
fn test_is_f64_with_non_number_value() {
    let null_value = Value::Null;
    assert!(!null_value.is_f64());

    let bool_value = Value::Bool(true);
    assert!(!bool_value.is_f64());

    let string_value = Value::String("example".to_string());
    assert!(!string_value.is_f64());

    let array_value = Value::Array(vec![Value::Null]);
    assert!(!array_value.is_f64());

    let object_value = Value::Object(Map::new());
    assert!(!object_value.is_f64());
}

#[test]
fn test_is_f64_with_number_value() {
    let int_value = Value::Number(Number::from_i64(42).unwrap());
    assert!(!int_value.is_f64());

    let uint_value = Value::Number(Number::from_u128(100u128).unwrap());
    assert!(!uint_value.is_f64());

    let float_value = Value::Number(Number::from_f64(3.14).unwrap());
    assert!(float_value.is_f64());
}

