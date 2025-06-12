// Answer 0

#[test]
fn test_eq_f64_with_nan() {
    let value = Value::Number(Number::from_f64(0.0).unwrap());
    assert_eq!(eq_f64(&value, f64::NAN), false);
}

#[test]
fn test_eq_f64_with_positive_value() {
    let value = Value::Number(Number::from_f64(3.14).unwrap());
    assert_eq!(eq_f64(&value, 3.14), true);
}

#[test]
fn test_eq_f64_with_different_positive_value() {
    let value = Value::Number(Number::from_f64(3.14).unwrap());
    assert_eq!(eq_f64(&value, 2.71), false);
}

#[test]
fn test_eq_f64_with_negative_value() {
    let value = Value::Number(Number::from_f64(-2.71).unwrap());
    assert_eq!(eq_f64(&value, -2.71), true);
}

#[test]
fn test_eq_f64_with_zero() {
    let value = Value::Number(Number::from_f64(0.0).unwrap());
    assert_eq!(eq_f64(&value, 0.0), true);
}

#[test]
fn test_eq_f64_with_zero_and_nan() {
    let value = Value::Number(Number::from_f64(0.0).unwrap());
    assert_eq!(eq_f64(&value, f64::NAN), false);
}

