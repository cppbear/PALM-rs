// Answer 0

#[test]
fn test_eq_f64_with_f64_value() {
    let value = Value::Number(Number::from_f64(3.14).unwrap());
    assert!(eq_f64(&value, 3.14));
    assert!(!eq_f64(&value, 2.71));
}

#[test]
fn test_eq_f64_with_null_value() {
    let value = Value::Null;
    assert!(!eq_f64(&value, 3.14));
}

#[test]
fn test_eq_f64_with_bool_value() {
    let value = Value::Bool(true);
    assert!(!eq_f64(&value, 1.0));
}

#[test]
fn test_eq_f64_with_non_number_value() {
    let value = Value::String("not a number".to_string());
    assert!(!eq_f64(&value, 3.14));
}

#[test]
fn test_eq_f64_with_float_number_value() {
    let value = Value::Number(Number::from_f64(0.0).unwrap());
    assert!(eq_f64(&value, 0.0));
    assert!(!eq_f64(&value, 1.0));
} 

#[test]
fn test_eq_f64_with_negative_f64_value() {
    let value = Value::Number(Number::from_f64(-5.0).unwrap());
    assert!(eq_f64(&value, -5.0));
    assert!(!eq_f64(&value, 5.0));
}

